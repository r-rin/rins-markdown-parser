use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Error as ioError, Write},
    path::Path,
};

use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

#[derive(Error, Debug)]
pub enum ErrorParse {
    #[error("An error occurrred while parsing: {0}")]
    ParsingError(String),
    #[error("A file error occurrred: {0}")]
    FileError(#[from] ioError),
}

pub fn str_to_html(text: &str) -> Result<Vec<String>, ErrorParse> {
    let parsed_pairs =
        parse_markdown(text).map_err(|e| ErrorParse::ParsingError(format!("{:?}", e)))?;

    let markdown_content = parsed_pairs
        .into_iter()
        .next()
        .ok_or_else(|| ErrorParse::ParsingError(String::from("Expected markdown got nothing")))?;

    let html_lines: Result<Vec<String>, ErrorParse> = markdown_content
        .into_inner()
        .map(|pair| to_html(pair))
        .collect();

    html_lines
}

fn to_html(pair: Pair<Rule>) -> Result<String, ErrorParse> {
    match pair.as_rule() {
        Rule::empty_line => Ok(String::from("<br/>")),
        Rule::heading1 => Ok(format!("<h1>{}</h1>", parse_heading(pair)?)),
        Rule::heading2 => Ok(format!("<h2>{}</h2>", parse_heading(pair)?)),
        Rule::heading3 => Ok(format!("<h3>{}</h3>", parse_heading(pair)?)),
        Rule::code_block => match parse_code_block(pair) {
            Ok((lang, content)) => Ok(format!(
                "<pre><code class=\"language-{}\">{}</code></pre>",
                lang, content
            )),
            Err(err) => Err(err),
        },
        Rule::quote => Ok(format!("<blockquote>{}</blockquote>", parse_quote(pair)?)),
        Rule::horizontal_rule => Ok(String::from("<hr>")),
        Rule::paragraph => Ok(format!("<p>{}</p>", parse_paragraph(pair)?)),
        Rule::inline_link => match parse_inline_link(pair) {
            Ok((link_text, url)) => Ok(format!("<a href=\"{}\">{}</a>", url, link_text)),
            Err(err) => Err(err),
        },
        Rule::inline_image => match parse_inline_image(pair) {
            Ok((alt_text, url)) => Ok(format!("<img src=\"{}\" alt=\"{}\">", url, alt_text)),
            Err(err) => Err(err),
        },
        Rule::bold => Ok(format!("<strong>{}</strong>", parse_styled_text(pair)?)),
        Rule::italic => Ok(format!("<em>{}</em>", parse_styled_text(pair)?)),
        Rule::strikethrough => Ok(format!("<del>{}</del>", parse_styled_text(pair)?)),
        Rule::underline => Ok(format!("<u>{}</u>", parse_styled_text(pair)?)),
        Rule::escaped => Ok(parse_escaped_char(pair)?),
        Rule::content => Ok(String::from(html_escape::encode_text(pair.as_str()))),
        Rule::plain_text => Ok(String::from(html_escape::encode_text(pair.as_str()))),
        Rule::EOI => Ok(String::new()),
        _ => Err(ErrorParse::ParsingError(format!(
            "Unknown rule: {:#?}",
            pair.as_rule()
        ))),
    }
}

fn parse_escaped_char(pair: Pair<Rule>) -> Result<String, ErrorParse> {
    let mut inner = pair.into_inner();
    let char_sym = inner
        .next()
        .ok_or_else(|| {
            ErrorParse::ParsingError(String::from("Expected char rule inside an escaped"))
        })?
        .as_str();

    let encoded_char = String::from(html_escape::encode_text(char_sym));
    Ok(encoded_char)
}

fn parse_styled_text(pair: Pair<Rule>) -> Result<String, ErrorParse> {
    let inner = pair.into_inner();
    let mut html_content = String::new();

    for rule in inner {
        let content = to_html(rule)?;
        html_content.push_str(content.as_str());
    }

    Ok(html_content)
}

fn parse_inline_image(pair: Pair<Rule>) -> Result<(String, String), ErrorParse> {
    let mut inner = pair.into_inner();
    let alt_text = inner
        .next()
        .ok_or_else(|| {
            ErrorParse::ParsingError(String::from(
                "Expected link_text rule inside an inline_link",
            ))
        })?
        .as_str();
    let url = inner
        .next()
        .ok_or_else(|| ErrorParse::ParsingError(String::from("Expected url inside a inline_link")))?
        .as_str();

    let alt_text_encoded = String::from(html_escape::encode_text(alt_text));
    let url_string = String::from(url);

    Ok((alt_text_encoded, url_string))
}

fn parse_inline_link(pair: Pair<Rule>) -> Result<(String, String), ErrorParse> {
    let mut inner = pair.into_inner();
    let link_text = inner
        .next()
        .ok_or_else(|| {
            ErrorParse::ParsingError(String::from(
                "Expected link_text rule inside an inline_link",
            ))
        })?
        .as_str();
    let link_url = inner
        .next()
        .ok_or_else(|| ErrorParse::ParsingError(String::from("Expected url inside a inline_link")))?
        .as_str();

    let link_text_encoded = String::from(html_escape::encode_text(link_text));
    let url_string = String::from(link_url);

    Ok((link_text_encoded, url_string))
}

fn parse_paragraph(pair: Pair<Rule>) -> Result<String, ErrorParse> {
    let inner_lines = pair.into_inner();
    let total_lines = inner_lines.len();
    let mut html_content = String::new();

    for (i, line) in inner_lines.enumerate() {
        match line.as_rule() {
            Rule::paragraph_line => {
                let line_content = line
                    .clone()
                    .into_inner()
                    .map(|text| to_html(text))
                    .collect::<Result<String, ErrorParse>>()?;
                html_content.push_str(line_content.as_str());

                if i < total_lines - 1 {
                    html_content.push_str("<br>");
                }
            }
            _ => {
                return Err(ErrorParse::ParsingError(format!(
                    "Unexpected rule inside a paragraph: {:#?}",
                    line.as_rule()
                )));
            }
        }
    }

    Ok(html_content)
}

fn parse_quote(pair: Pair<Rule>) -> Result<String, ErrorParse> {
    let mut inner = pair.into_inner();
    let paragraph = inner.next().ok_or_else(|| {
        ErrorParse::ParsingError(String::from("Expected paragraph rule inside a quote"))
    })?;
    to_html(paragraph)
}

fn parse_code_block(pair: Pair<Rule>) -> Result<(String, String), ErrorParse> {
    let mut inner = pair.into_inner();
    let code_lang = inner
        .next()
        .ok_or_else(|| {
            ErrorParse::ParsingError(String::from("Expected code_lang rule iside an code block"))
        })?
        .as_str();
    let code_content = inner
        .next()
        .ok_or_else(|| {
            ErrorParse::ParsingError(String::from("Expected code content inside a code block"))
        })?
        .as_str();

    let code_encoded = String::from(html_escape::encode_text(code_lang));
    let content_encoded = String::from(html_escape::encode_text(code_content));

    Ok((code_encoded, content_encoded))
}

fn parse_heading(pair: Pair<Rule>) -> Result<String, ErrorParse> {
    let mut inner = pair.into_inner();
    let sngl_line_text = inner.next().ok_or_else(|| {
        ErrorParse::ParsingError(String::from(
            "Expected single_line_text rule inside a header",
        ))
    })?;

    let text: &str = sngl_line_text.as_str();

    Ok(String::from(html_escape::encode_text(text)))
}

pub fn parse_markdown(input: &str) -> Result<Pairs<Rule>, ErrorParse> {
    return Grammar::parse(Rule::markdown, input)
        .map_err(|err| ErrorParse::ParsingError(err.to_string()));
}

pub fn parse_by_rule(rule: Rule, input: &str) -> Result<Pairs<Rule>, ErrorParse> {
    return Grammar::parse(rule, input).map_err(|err| ErrorParse::ParsingError(err.to_string()));
}

pub fn md_to_html_file(md_path: &Path, html_path: &Path) -> Result<(), ErrorParse> {
    let file = File::open(md_path).map_err(ErrorParse::FileError)?;
    let reader = BufReader::new(file);

    let mut markdown_content = String::new();
    for line in reader.lines() {
        markdown_content.push_str(&line?);
        markdown_content.push('\n');
    }

    let html_lines = str_to_html(&markdown_content)?;

    let mut output_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(html_path)
        .map_err(ErrorParse::FileError)?;

    for line in html_lines {
        output_file
            .write_all(line.as_bytes())
            .map_err(ErrorParse::FileError)?;
        output_file
            .write_all(b"\n")
            .map_err(ErrorParse::FileError)?;
    }

    Ok(())
}

pub fn parse_to_console(text: &str) -> Result<(), ErrorParse> {
    let res = str_to_html(text)?;

    for line in res {
        println!("{}", line);
    }

    Ok(())
}

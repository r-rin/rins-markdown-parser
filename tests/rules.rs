use rins_markdown_parser::*;

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::*;
    use std::result::Result::Ok;

    #[test]
    fn check_headers() {
        check_header(Rule::heading1, "# Header 1 ###   some text!");
        check_header(Rule::heading1, "# Header 1 # !## some !@#$%^&*(*&^%|}\"?text!");
        check_header(Rule::heading1, "# M");

        check_header(Rule::heading2, "## Header 2 ## Some other     text.");
        check_header(Rule::heading2, "## Special # symbols in header 2! @#$%");
        check_header(Rule::heading2, "## Another heading 2");

        check_header(Rule::heading3, "### Header 3 with          more text");
        check_header(Rule::heading3, "### H3 with special chars $%^&*()");
        check_header(Rule::heading3, "### Simple header 3");
    }

    #[test]
    #[should_panic]
    fn check_wrong_headers() {
        check_header(Rule::heading1, "# ");
        check_header(Rule::heading1, "#");
        check_header(Rule::heading1, "## Another header, not h1");

        check_header(Rule::heading2, "## ");
        check_header(Rule::heading2, "##");
        check_header(Rule::heading2, "# Header not h2");

        check_header(Rule::heading3, "### ");
        check_header(Rule::heading3, "###");
        check_header(Rule::heading3, "# Header not h3");
    }

    #[test]
    fn check_plain_text() {
        let res1 = parse_by_rule(Rule::plain_text, "This is a plain text");
        assert!(res1.is_ok());
        assert_eq!(res1.unwrap().as_str(), "This is a plain text");

        let res2 = parse_by_rule(Rule::plain_text, "This is \\* plain text");
        assert!(res2.is_ok());
        assert_eq!(res2.unwrap().as_str(), "This is ");
    }

    #[test]
    fn check_escaped() -> Result<()> {
        {
            let input = "\\*";
            let mut pairs = parse_by_rule(Rule::escaped, input)?;
            let pair = pairs.next().ok_or_else(|| anyhow!("Expected a pair, but found none"))?;
            let char_pair = pair
                .into_inner()
                .next()
                .ok_or_else(|| anyhow!("Expected an inner pair, but found none"))?;
            assert_eq!(char_pair.as_str(), "*");
        }

        {
            let input = "\\\\";
            let mut pairs = parse_by_rule(Rule::escaped, input)?;
            let pair = pairs.next().ok_or_else(|| anyhow!("Expected a pair, but found none"))?;
            let char_pair = pair
                .into_inner()
                .next()
                .ok_or_else(|| anyhow!("Expected a pair with char rule, but found none"))?;
            assert_eq!(char_pair.as_str(), "\\");
        }

        return Ok(());
    }

    #[test]
    #[should_panic]
    fn check_escaped_panic() {
        let input = "\\ a";
        let pairs = parse_by_rule(Rule::escaped, input);
        pairs.expect("An error occurred, expected cause: whitespace after \\");
    }

    #[test]
    fn check_italic() -> Result<()> {
        {
            let input = "*this text is italic*";
            let mut pairs = parse_by_rule(Rule::italic, input)?;
            let pair = pairs.next().ok_or_else(|| anyhow!("Expected a pair, but found none"))?;
            assert_eq!(pair.as_rule(), Rule::italic);
            let content = pair
                .into_inner()
                .next()
                .ok_or_else(|| anyhow!("Expected a pair with content rule, but found none"))?;
            assert_eq!(content.as_str(), "this text is italic");
        }

        {
            let input = "_italic test 2_";
            let mut pairs = parse_by_rule(Rule::italic, input)?;
            let pair = pairs.next().ok_or_else(|| anyhow!("Expected a pair, but found none"))?;
            assert_eq!(pair.as_rule(), Rule::italic);
            let content = pair
                .into_inner()
                .next()
                .ok_or_else(|| anyhow!("Expected a pair with content rule, but found none"))?;
            assert_eq!(content.as_str(), "italic test 2");
        }

        Ok(())
    }

    #[test]
    fn check_bold() -> Result<()> {
        let input = "**this text is bold**";
        let mut pairs = parse_by_rule(Rule::bold, input)?;
        let pair = pairs.next().ok_or_else(|| anyhow!("Expected a pair, but found none"))?;
        assert_eq!(pair.as_rule(), Rule::bold);
        let content = pair
            .into_inner()
            .next()
            .ok_or_else(|| anyhow!("Expected a pair with content rule, but found none"))?;
        assert_eq!(content.as_str(), "this text is bold");

        Ok(())
    }

    #[test]
    fn check_underline() -> Result<()> {
        let input = "__underlined text!__";
        let mut pairs = parse_by_rule(Rule::underline, input)?;
        let pair = pairs.next().ok_or_else(|| anyhow!("Expected a pair, but found none"))?;
        assert_eq!(pair.as_rule(), Rule::underline);
        let content = pair
            .into_inner()
            .next()
            .ok_or_else(|| anyhow!("Expected a pair with content rule, but found none"))?;
        assert_eq!(content.as_str(), "underlined text!");

        Ok(())
    }

    #[test]
    fn check_strikethrough() -> Result<()> {
        let input = "~~some striked text!~~";
        let mut pairs = parse_by_rule(Rule::strikethrough, input)?;
        let pair = pairs.next().ok_or_else(|| anyhow!("Expected a pair, but found none"))?;
        assert_eq!(pair.as_rule(), Rule::strikethrough);
        let content = pair
            .into_inner()
            .next()
            .ok_or_else(|| anyhow!("Expected a pair with content rule, but found none"))?;
        assert_eq!(content.as_str(), "some striked text!");

        Ok(())
    }

    #[test]
    fn check_inline_link() -> Result<()> {
        let input = "[click this!](https://google.com/)";
        let mut pairs = parse_by_rule(Rule::inline_link, input)?;
        let pair = pairs.next().ok_or_else(|| anyhow!("Expected a pair, but found none"))?;
        assert_eq!(pair.as_rule(), Rule::inline_link);
        
        let mut inner_iter = pair.into_inner();
        let link_text = inner_iter
            .next()
            .ok_or_else(|| anyhow!("Expected a pair with link_text rule, but found none"))?;
        assert_eq!(link_text.as_rule(), Rule::link_text);
        assert_eq!(link_text.as_str(), "click this!");

        let url = inner_iter
            .next()
            .ok_or_else(|| anyhow!("Expected a pair with url rule, but found none"))?;
        assert_eq!(url.as_rule(), Rule::url);
        assert_eq!(url.as_str(), "https://google.com/");

        Ok(())
    }

    #[test]
    fn check_inline_image() -> Result<()> {
        let input = "![alternative text](https://example.com/image.png)";
        let mut pairs = parse_by_rule(Rule::inline_image, input)?;
        let pair = pairs.next().ok_or_else(|| anyhow!("Expected a pair, but found none"))?;
        assert_eq!(pair.as_rule(), Rule::inline_image);
        
        let mut inner_iter = pair.into_inner();
        let alt_text = inner_iter
            .next()
            .ok_or_else(|| anyhow!("Expected a pair with alt_text rule, but found none"))?;
        assert_eq!(alt_text.as_rule(), Rule::alt_text);
        assert_eq!(alt_text.as_str(), "alternative text");

        let url = inner_iter
            .next()
            .ok_or_else(|| anyhow!("Expected a pair with url rule, but found none"))?;
        assert_eq!(url.as_rule(), Rule::url);
        assert_eq!(url.as_str(), "https://example.com/image.png");

        Ok(())
    }

    #[test]
    fn check_mixed_styled() -> Result<()> {
        let input = "_hi**~~[link](https://example.org/)some text~~fgh\\***_";
        let mut pairs = parse_by_rule(Rule::styled_text, input)?;
        
        let pair = pairs.next().ok_or_else(|| anyhow!("Expected a pair, but found none"))?;
        assert_eq!(pair.as_rule(), Rule::italic);

        let mut inner_iter = pair.into_inner();

        let content = inner_iter
            .next()
            .ok_or_else(|| anyhow!("Expected a pair with content rule, but found none"))?;
        assert_eq!(content.as_rule(), Rule::content);
        assert_eq!(content.as_str(), "hi");

        let bold = inner_iter
            .next()
            .ok_or_else(|| anyhow!("Expected a pair with bold rule, but found none"))?;
        assert_eq!(bold.as_rule(), Rule::bold);

        let mut bold_inner = bold.into_inner();

        let strikethrough = bold_inner
            .next()
            .ok_or_else(|| anyhow!("Expected a pair with strikethrough rule, but found none"))?;
        assert_eq!(strikethrough.as_rule(), Rule::strikethrough);

        let mut strike_inner = strikethrough.into_inner();

        let inline_link = strike_inner
            .next()
            .ok_or_else(|| anyhow!("Expected a pair with inline_link rule, but found none"))?;
        assert_eq!(inline_link.as_rule(), Rule::inline_link);

        let mut link_inner = inline_link.into_inner();

        let link_text = link_inner
            .next()
            .ok_or_else(|| anyhow!("Expected a pair with link_text rule, but found none"))?;
        assert_eq!(link_text.as_rule(), Rule::link_text);
        assert_eq!(link_text.as_str(), "link");

        let url = link_inner
            .next()
            .ok_or_else(|| anyhow!("Expected a pair with url rule, but found none"))?;
        assert_eq!(url.as_rule(), Rule::url);
        assert_eq!(url.as_str(), "https://example.org/");

        let content = strike_inner
            .next()
            .ok_or_else(|| anyhow!("Expected a pair with content rule, but found none"))?;
        assert_eq!(content.as_rule(), Rule::content);
        assert_eq!(content.as_str(), "some text");

        let bold_content = bold_inner
            .next()
            .ok_or_else(|| anyhow!("Expected a pair with content rule inside bold, but found none"))?;
        assert_eq!(bold_content.as_rule(), Rule::content);
        assert_eq!(bold_content.as_str(), "fgh");

        let escaped = bold_inner
            .next()
            .ok_or_else(|| anyhow!("Expected a pair with escaped rule, but found none"))?;
        assert_eq!(escaped.as_rule(), Rule::escaped);

        let mut escaped_inner = escaped.into_inner();
        let char_pair = escaped_inner
            .next()
            .ok_or_else(|| anyhow!("Expected a pair with char rule, but found none"))?;
        assert_eq!(char_pair.as_rule(), Rule::char);
        assert_eq!(char_pair.as_str(), "*");

        Ok(())
    }

    #[test]
    fn check_paragraph() -> Result<()> {
        let input = "This is the **first** line!\nThis one is _second_";
        let mut pairs = parse_by_rule(Rule::paragraph, input)?;
        
        let pair = pairs.next().ok_or_else(|| anyhow!("Expected a paragraph, but found none"))?;
        assert_eq!(pair.as_rule(), Rule::paragraph);

        let mut paragraph_inner = pair.into_inner();

        let line1 = paragraph_inner
            .next()
            .ok_or_else(|| anyhow!("Expected the first paragraph_line, but found none"))?;
        assert_eq!(line1.as_rule(), Rule::paragraph_line);

        let mut line1_inner = line1.into_inner();

        let plain1 = line1_inner
            .next()
            .ok_or_else(|| anyhow!("Expected plain_text in the first line, but found none"))?;
        assert_eq!(plain1.as_rule(), Rule::plain_text);
        assert_eq!(plain1.as_str(), "This is the ");

        let bold = line1_inner
            .next()
            .ok_or_else(|| anyhow!("Expected bold in the first line, but found none"))?;
        assert_eq!(bold.as_rule(), Rule::bold);

        let mut bold_inner = bold.into_inner();
        let bold_content = bold_inner
            .next()
            .ok_or_else(|| anyhow!("Expected bold content, but found none"))?;
        assert_eq!(bold_content.as_rule(), Rule::content);
        assert_eq!(bold_content.as_str(), "first");

        let plain2 = line1_inner
            .next()
            .ok_or_else(|| anyhow!("Expected plain_text after bold in the first line, but found none"))?;
        assert_eq!(plain2.as_rule(), Rule::plain_text);
        assert_eq!(plain2.as_str(), " line!");

        let line2 = paragraph_inner
            .next()
            .ok_or_else(|| anyhow!("Expected the second paragraph_line, but found none"))?;
        assert_eq!(line2.as_rule(), Rule::paragraph_line);

        let mut line2_inner = line2.into_inner();

        let plain3 = line2_inner
            .next()
            .ok_or_else(|| anyhow!("Expected plain_text in the second line, but found none"))?;
        assert_eq!(plain3.as_rule(), Rule::plain_text);
        assert_eq!(plain3.as_str(), "This one is ");

        let italic = line2_inner
            .next()
            .ok_or_else(|| anyhow!("Expected italic in the second line, but found none"))?;
        assert_eq!(italic.as_rule(), Rule::italic);

        let mut italic_inner = italic.into_inner();
        let italic_content = italic_inner
            .next()
            .ok_or_else(|| anyhow!("Expected italic content, but found none"))?;
        assert_eq!(italic_content.as_rule(), Rule::content);
        assert_eq!(italic_content.as_str(), "second");

        Ok(())
    }


    #[test]
    fn check_quote() -> Result<()> {
        let input = ">This is a text in a quote\nthis is also a part of a quote\nthis one is too.";
        let mut pairs = parse_by_rule(Rule::quote, input)?;
    
        let pair = pairs.next().ok_or_else(|| anyhow!("Expected a quote, but found none"))?;
        assert_eq!(pair.as_rule(), Rule::quote);
    
        let mut quote_inner = pair.into_inner();
        let paragraph = quote_inner
            .next()
            .ok_or_else(|| anyhow!("Expected a paragraph within the quote, but found none"))?;
        assert_eq!(paragraph.as_rule(), Rule::paragraph);
    
        let mut paragraph_inner = paragraph.into_inner();
    
        let line1 = paragraph_inner
            .next()
            .ok_or_else(|| anyhow!("Expected the first paragraph_line, but found none"))?;
        assert_eq!(line1.as_rule(), Rule::paragraph_line);
    
        let plain_text1 = line1
            .into_inner()
            .next()
            .ok_or_else(|| anyhow!("Expected plain_text in the first line, but found none"))?;
        assert_eq!(plain_text1.as_rule(), Rule::plain_text);
        assert_eq!(plain_text1.as_str(), "This is a text in a quote");
    
        let line2 = paragraph_inner
            .next()
            .ok_or_else(|| anyhow!("Expected the second paragraph_line, but found none"))?;
        assert_eq!(line2.as_rule(), Rule::paragraph_line);
    
        let plain_text2 = line2
            .into_inner()
            .next()
            .ok_or_else(|| anyhow!("Expected plain_text in the second line, but found none"))?;
        assert_eq!(plain_text2.as_rule(), Rule::plain_text);
        assert_eq!(plain_text2.as_str(), "this is also a part of a quote");
    
        let line3 = paragraph_inner
            .next()
            .ok_or_else(|| anyhow!("Expected the third paragraph_line, but found none"))?;
        assert_eq!(line3.as_rule(), Rule::paragraph_line);
    
        let plain_text3 = line3
            .into_inner()
            .next()
            .ok_or_else(|| anyhow!("Expected plain_text in the third line, but found none"))?;
        assert_eq!(plain_text3.as_rule(), Rule::plain_text);
        assert_eq!(plain_text3.as_str(), "this one is too.");
    
        Ok(())
    }
    

    #[test]
    fn check_code_block() -> Result<()> {
        let input = "```py\nprint(\"Hello World!\")\n```";
        let mut pairs = parse_by_rule(Rule::code_block, input)?;
    
        let pair = pairs.next().ok_or_else(|| anyhow!("Expected a code_block, but found none"))?;
        assert_eq!(pair.as_rule(), Rule::code_block);
    
        let mut code_block_inner = pair.into_inner();
    
        let code_lang = code_block_inner
            .next()
            .ok_or_else(|| anyhow!("Expected a code_lang, but found none"))?;
        assert_eq!(code_lang.as_rule(), Rule::code_lang);
        assert_eq!(code_lang.as_str(), "py");
    
        let code_content = code_block_inner
            .next()
            .ok_or_else(|| anyhow!("Expected code_content, but found none"))?;
        assert_eq!(code_content.as_rule(), Rule::code_content);
        assert_eq!(code_content.as_str(), "print(\"Hello World!\")");
    
        Ok(())
    }
    
    #[test]
    fn check_horizontal_rule() -> Result<()> {
        let inputs = vec![
            "---",
            "***",
            "–––",
            "---   ",
            "***\n",
            "–––  \n",
        ];

        for input in inputs {
            let mut pairs = parse_by_rule(Rule::horizontal_rule, input)?;
            let pair = pairs.next().ok_or_else(|| anyhow!("Expected a horizontal_rule, but found none"))?;
            assert_eq!(pair.as_rule(), Rule::horizontal_rule);
            assert_eq!(pair.as_str().trim(), input.trim());
        }

        Ok(())
    }

    #[test]
    fn check_markdown() -> Result<()> {
        let input = "# Hello this is my 1st post!\n–––\n\nThis code prints \"Hello world\":\n```py\nprint(\"Hello world!\")\n```\n\nThis is **bold** text!\nThat's all. Bye!";
    
        let mut pairs = parse_by_rule(Rule::markdown, input)?;
    
        let pair = pairs.next().ok_or_else(|| anyhow!("Expected a markdown root, but found none"))?;
        assert_eq!(pair.as_rule(), Rule::markdown);
    
        let mut markdown_inner = pair.into_inner();
    
        let heading1 = markdown_inner
            .next()
            .ok_or_else(|| anyhow!("Expected heading1, but found none"))?;
        assert_eq!(heading1.as_rule(), Rule::heading1);
        let heading_text = heading1.into_inner().next().ok_or_else(|| anyhow!("Expected heading text"))?;
        assert_eq!(heading_text.as_str(), "Hello this is my 1st post!");
    
        let horizontal_rule = markdown_inner
            .next()
            .ok_or_else(|| anyhow!("Expected horizontal_rule, but found none"))?;
        assert_eq!(horizontal_rule.as_rule(), Rule::horizontal_rule);
        assert_eq!(horizontal_rule.as_str().trim(), "–––");
    
        let empty_line1 = markdown_inner
            .next()
            .ok_or_else(|| anyhow!("Expected empty_line, but found none"))?;
        assert_eq!(empty_line1.as_rule(), Rule::empty_line);
    
        let paragraph1 = markdown_inner
            .next()
            .ok_or_else(|| anyhow!("Expected paragraph, but found none"))?;
        assert_eq!(paragraph1.as_rule(), Rule::paragraph);
        let mut paragraph1_inner = paragraph1.into_inner();
    
        let paragraph_line1 = paragraph1_inner
            .next()
            .ok_or_else(|| anyhow!("Expected paragraph_line, but found none"))?;
        assert_eq!(paragraph_line1.as_rule(), Rule::paragraph_line);
        let plain_text1 = paragraph_line1
            .into_inner()
            .next()
            .ok_or_else(|| anyhow!("Expected plain_text in the first paragraph_line"))?;
        assert_eq!(plain_text1.as_str(), "This code prints \"Hello world\":");
    
        let code_block = markdown_inner
            .next()
            .ok_or_else(|| anyhow!("Expected code_block, but found none"))?;
        assert_eq!(code_block.as_rule(), Rule::code_block);
        let mut code_block_inner = code_block.into_inner();
    
        let code_lang = code_block_inner
            .next()
            .ok_or_else(|| anyhow!("Expected code_lang, but found none"))?;
        assert_eq!(code_lang.as_rule(), Rule::code_lang);
        assert_eq!(code_lang.as_str(), "py");
    
        let code_content = code_block_inner
            .next()
            .ok_or_else(|| anyhow!("Expected code_content, but found none"))?;
        assert_eq!(code_content.as_rule(), Rule::code_content);
        assert_eq!(code_content.as_str(), "print(\"Hello world!\")");
    
        let empty_line2 = markdown_inner
            .next()
            .ok_or_else(|| anyhow!("Expected empty_line, but found none"))?;
        assert_eq!(empty_line2.as_rule(), Rule::empty_line);
    
        let paragraph2 = markdown_inner
            .next()
            .ok_or_else(|| anyhow!("Expected paragraph, but found none"))?;
        assert_eq!(paragraph2.as_rule(), Rule::paragraph);
        let mut paragraph2_inner = paragraph2.into_inner();
    
        let paragraph_line2 = paragraph2_inner
            .next()
            .ok_or_else(|| anyhow!("Expected paragraph_line, but found none"))?;
        assert_eq!(paragraph_line2.as_rule(), Rule::paragraph_line);
        let mut paragraph_line2_inner = paragraph_line2.into_inner();
        let plain_text2 = paragraph_line2_inner
            .next()
            .ok_or_else(|| anyhow!("Expected plain_text, but found none"))?;
        assert_eq!(plain_text2.as_str(), "This is ");
        let bold = paragraph_line2_inner
            .next()
            .ok_or_else(|| anyhow!("Expected bold content, but found none"))?;
        assert_eq!(bold.as_rule(), Rule::bold);
        let bold_content = bold.into_inner().next().ok_or_else(|| anyhow!("Expected bold content text"))?;
        assert_eq!(bold_content.as_str(), "bold");
        let plain_text3 = paragraph_line2_inner
            .next()
            .ok_or_else(|| anyhow!("Expected plain_text, but found none"))?;
        assert_eq!(plain_text3.as_str(), " text!");
    
        let paragraph_line3 = paragraph2_inner
            .next()
            .ok_or_else(|| anyhow!("Expected paragraph_line, but found none"))?;
        assert_eq!(paragraph_line3.as_rule(), Rule::paragraph_line);
        let plain_text4 = paragraph_line3
            .into_inner()
            .next()
            .ok_or_else(|| anyhow!("Expected plain_text, but found none"))?;
        assert_eq!(plain_text4.as_str(), "That's all. Bye!");
    
        println!("{:#?}", markdown_inner);
        let eoi = markdown_inner
            .next()
            .ok_or_else(|| anyhow!("Expected end of input, but found none"))?;
        assert_eq!(eoi.as_rule(), Rule::EOI);
    
        Ok(())
    }    
}

fn get_headers_start_length(rule: Rule) -> usize {
    match rule {
        Rule::heading1 => {1}
        Rule::heading2 => {2}
        Rule::heading3 => {3}
        _ => {0}
    }
}

fn check_header(rule: Rule, header_string: &str) {
    let result = 
        parse_by_rule(rule, header_string)
        .expect("An error occured while parsing");
    
    for pair in result {
        if pair.as_rule() == Rule::heading1 {  

            // Checking if the whole string passed
            println!("Read: {}", pair.as_str());
            assert_eq!(header_string, pair.as_str());
    
            let inner_pairs = pair.into_inner();
            let header_text: Vec<&str> = inner_pairs
                .filter(|pair| pair.as_rule() == Rule::single_line_text)
                .map(|pair| pair.as_str())
                .collect();
        
            for text in &header_text {
                println!("single_line_text content: {}", *text);
                let string_start: usize = get_headers_start_length(rule) + 1;
                assert_eq!(*text, &header_string[string_start..]);
            }
        }
    }
}
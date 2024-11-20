use clap::{Arg, Command};
use rins_markdown_parser::{md_to_html_file, parse_to_console, ErrorParse};
use std::path::PathBuf;

fn main() -> Result<(), ErrorParse> {
    let matches = Command::new("rins_markdown_parser")
        .version("0.1.0")
        .author("r-rin")
        .about("Allows to interact with markdown parser via a Command Line Interface.")
        .subcommand(
            Command::new("parse")
                .about("Parses provided markdown text and returns it in html format")
                .arg(
                    Arg::new("input_file")
                        .short('I')
                        .long("in")
                        .help("Specifies the location of the file from which the markdown text will be read")
                )
                .arg(
                    Arg::new("output_file")
                        .short('O')
                        .long("out")
                        .help("Defines the location of the file to which the result of conversion to html will be saved")
                )
                .arg(
                    Arg::new("text")
                        .short('t')
                        .long("text")
                        .help("Accepts text in markdown format from the console")
                        .conflicts_with("input_file")
                        .conflicts_with("output_file")
                )
        )
        .subcommand(
            Command::new("credits")
                .about("Displays credits and project information")
        ).override_help(
"rins_markdown_parser v0.1.0
Allows to interact with markdown parser via a Command Line Interface.
Note: contains bugs 🐞

Usage: rins_markdown_parser [COMMAND]

Commands:
  parse    Parses provided markdown text and returns it in html format
  credits  Displays credits and project information
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version"
        )
        .get_matches();

    match matches.subcommand() {
        Some(("parse", sub_m)) => {
            if let Some(text) = sub_m.get_one::<String>("text") {
                parse_to_console(text)?;
                return Ok(());
            }

            let input_path_string = sub_m
                .get_one::<String>("input_file")
                .expect("If text is absent, then input_file is required");
            let output_path_string = sub_m
                .get_one::<String>("output_file")
                .expect("If text is absent, then output_file is required");

            let input_path = PathBuf::from(input_path_string);
            let output_path = PathBuf::from(output_path_string);

            md_to_html_file(&input_path, &output_path)?;
        }
        Some(("credits", _)) => {
            println!("CREDITS\n\nThis parser was developed as part of the Rust Programming Language course at NaUKMA with the support of the Ukrainian Rust community.\nGrammar is far from an ideal one, use with caution.")
        }
        _ => println!("Unknown command encountered. Use help subcommand for more information."),
    }

    Ok(())
}

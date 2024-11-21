# Simple Markdown Parser

> [!IMPORTANT]
> This parser uses not the most ideal grammar, so it is recommended **to avoid** using any complex or ambiguous combinations of styles, etc. For example, if you use bold and italic styles at the same time, then it is recommended to use underscores (`_italic_`) for italic styling.
 
**Crates.io**: [click here](https://crates.io/crates/rins_markdown_parser)
**Github**: [click here](https://github.com/r-rin/rins-markdown-parser)

---

This is a Rust library that parses Markdown text, covering essential Markdown syntax elements such as headers, lists, emphasis, links, code blocks, and more. It parses Markdown into an Abstract Syntax Tree (AST), making it easier to manipulate, transform, or render Markdown content in various formats.

### Features
* Headers - Parses Markdown headers (`#`, `##`, `###`, etc.) into structured nodes in the AST.
* Emphasis - Recognizes *italic* and **bold** text, along with other emphasis markers.
* Links - Parses inline links (`[text](url)`) and reference links.
* Code Blocks - Detects inline code (`code`) and fenced code blocks.
* Blockquotes - Parses quoted text (`> Quote`) as distinct elements.
* Images - Recognizes inline images (`![alt text](url)`).
* Horizontal Rule - Detects horizontal rules `---` in your file.

### Plans
* Lists - Support for both ordered (`1. Item`) and unordered (`- Item or * Item`) lists.
* Tables - Recognition of tables with rows and columns.
* Footnotes - Support for footnotes, allowing references in the text and corresponding notes at the bottom.
* Task List - Parsing of task lists, with checkboxes (e.g., `- [ ] Task or - [x] Done`).
* Emoji - Recognition of shortcodes for emojis (e.g., `:smile:`) and converting them to the appropriate Unicode or image representation.
* Highlighted Text - Support for highlighted text (e.g., using `==highlighted==`).
* Subscript - Parsing of subscript text (e.g., `H~2~O`).
* Superscript - Parsing of superscript text (e.g., `X^2^`).
* Definition Lists - Parsing of definition lists.
* Markdown extensions - Support for extended Markdown features like GitHub-flavored Markdown (GFM), including task lists, strikethrough, and more.

The parser processes Markdown into an Abstract Syntax Tree, which can be used for rendering Markdown as HTML, analyzing document structure, or exporting to other formats or editing a markdown file.

# Grammar

## 1. General Structure

```pest
markdown = { SOI ~ (block ~ empty_line*)* ~ EOI }

block = _{
  heading
  | quote
  | code_block
  | horizontal_rule
  | paragraph
}
```

- The file starts with the **Start of Input** (`SOI`) and ends with the **End of Input** (`EOI`).
- Markdown documents are composed of **blocks** separated by zero or more **empty lines**.

## 2. Block Elements
### 2.1 Headings

```pest
heading = _{
    heading1
  | heading2
  | heading3
}

heading1 = {
    "#" ~ ws ~ single_line_text ~ NEWLINE?
}

heading2 = {
    "##" ~ ws ~ single_line_text ~ NEWLINE? 
}

heading3 = {
    "###" ~ ws ~ single_line_text ~ NEWLINE?
}

single_line_text = {
    (!NEWLINE ~ ANY)+
}
```

- Represented using one or more `#` symbols at the start of the line.
- One `#` corresponds to Heading 1, two `##` to Heading 2, and three `###` to Heading 3.
- Must be followed by a space and a single line of text.
- Example:
```md
# Heading 1
## Heading 2
### Heading 3
```

### 2.2 Horizontal Rules

```pest
horizontal_rule = {
    ("---"|"***"|"–––") ~ ws* ~ (NEWLINE | EOI)
}    
```

- Created with three or more of the following symbols:
  - Dashes (`---`)
  - Asterisks (`***`)
  - En-dashes (`–––`)
- Can optionally include trailing whitespace and must end with a newline.
- Example: 
```md
---
***
–––
```

### 2.3 Blockquotes

```pest
quote =  {
    ">" ~ paragraph
}
```

- Indicated by a `>` character followed by a paragraph.
- Example:
```md
> This is a quote. Hello!
```

### 2.4 Code Blocks

```pest
code_block = {
    "```" ~ (code_lang ~ ws* ~ NEWLINE)? ~ code_content ~ NEWLINE? ~ "```" ~ NEWLINE?
}

code_lang = {
    ws* ~ ('a'..'z' | 'A'..'Z')+
}

code_content = {
    (!(NEWLINE? ~ "```") ~ ANY)+
}
```

- Start and end with three backticks (` ``` `).
- May optionally include a programming language name after the opening backticks.
- Example:
```md
    ```py
        print("Hello World!")
    ```
```

### 2.5 Paragraphs

```pest
paragraph = {
	paragraph_line+ 
}

paragraph_line = {
	text+ ~ paragraph_break?
}

paragraph_break = _{
    NEWLINE
}

text = _{
    plain_text
  | escaped
  | styled_text
}
```

- Consist of one or more paragraph lines.
- Paragraphs are separated by an empty line or a paragraph break (newline).

## 3. Inline Elements

### 3.1 Text Styles

```pest
styled_text = _{
    escaped* ~ (bold | underline | italic | strikethrough | inline_image | inline_link | content) ~ escaped*
}

strikethrough = {
    "~~" ~ (styled_text)+ ~ "~~"
}

underline = {
    "__" ~ (styled_text)+ ~ "__"
}

bold = {
    "**" ~ (styled_text)+ ~ "**"
}

italic = {
    ("*" ~ (styled_text)+ ~ "*")
  | ("_" ~ (styled_text)+ ~ "_")
}

content = @{
    (!(exclude_styles | exclude_block_elems) ~ ANY)+
}
```

- **Bold:** Enclosed in double asterisks (`**`).
- **Italic:** Enclosed in single asterisks (`*`) or underscores (`_`).
- **Underline:** Enclosed in double underscores (`__`).
- **Strikethrough:** Enclosed in double tildes (`~~`).
- **Content** contains text which those elements are styling, used as plain text within styled elements.

### 3.2 Links

```pest
inline_link = {
	"[" ~ link_text ~ "](" ~ url ~ ")"
}

link_text = {
	(!"]" ~ ANY)+
}

url = {
	(!")" ~ ANY)+
}
```

- Formatted as `[link text](url)`.

### 3.3 Images

```pest
inline_image = {
	"![" ~ alt_text ~ "](" ~ url ~ ")"
}

alt_text = {
	(!"]" ~ ANY)+
}

url = {
	(!")" ~ ANY)+
}
```

- Formatted as `![alt text](url)`.

### 3.4 Escaped Characters

```pest
escaped = {
    "\\" ~ (!ws ~ char)
}

char = {
	ANY
}
```

- Special characters can be escaped using a backslash (`\`).

## 4. Miscellaneous Rules

### 4.1 Plain Text

```pest
plain_text = @{
    !exclude_block_elems ~ (!exclude_styles ~ ANY)+
}
```

- Any text not enclosed by styled elements or part of block elements.

### 4.2 Empty Lines

```pest
empty_line = {
	NEWLINE
}
```

- Represented by a single newline (`\n`).

> [!NOTE]
> More additional rules and their description can be found in the `src/grammar.pest`!

# Installation

### Using as a Crate

You can add this project as a dependency to your Rust project by fetching it from [crates.io](https://crates.io).

1. Open the folder of your desired project in a terminal.
2. Use `cargo add` to add the crate to your project's dependencies:
```bash
$ cargo add rins_markdown_parser
```
3. Import crate in any `.rs` file inside your project:
```rust
// any .rs file, e.g. main.rs
use rins_markdown_parser::{Grammar, parse_to_console} 
```

### Using as a Command-Line Tool

Alternatively, you can use this project as a standalone command-line interface (CLI). To do so:

1. Clone the repository:
```bash
$ git clone https://github.com/r-rin/rins-markdown-parser.git
$ cd rins-markdown-parser
```

2. Build the project:
```bash
$ cargo build --release
```
The compiled binary will be located in the target/release directory.

3. Run the CLI to parse a Markdown file:
```bash
$ ./target/release/rins-markdown-parser help
```

or use **make**

```bash
$ make run args="..."
```

# Usage
Crate provides various utilities for parsing Markdown text and converting it into HTML. Below are examples and explanations of how to use the provided functions.

### 1. Parse Markdown to HTML (String Input)

You can use the `str_to_html` function to parse Markdown text from a string and convert it to HTML.

```rust
use rins_markdown_parser::{str_to_html, ErrorParse};

fn main() -> Result<(), ErrorParse> {
    let markdown_text = "# Hello, World!\nThis is **bold** and *italic*.";
    let html_lines = str_to_html(markdown_text)?;

    for line in html_lines {
        println!("{}", line);
    }
    Ok(())
}
```
**Output:**
```html
<h1>Hello, World!</h1>
<p>This is <strong>bold</strong> and <em>italic</em>.</p>
```

### 2. Parse Markdown File to HTML File

Use the `md_to_html_file function` to convert a Markdown file into an HTML file.

```rust
use rins_markdown_parser::{md_to_html_file, ErrorParse};
use std::path::Path;

fn main() -> Result<(), ErrorParse> {
    let markdown_path = Path::new("example.md");
    let html_path = Path::new("example.html");

    md_to_html_file(markdown_path, html_path)?;
    println!("Markdown converted to HTML successfully!");

    Ok(())
}
```

### 3. Parse Markdown and Print HTML to Console

The `parse_to_console` function allows you to parse Markdown text and print the resulting HTML directly to the console.

```rust
use rins_markdown_parser::parse_to_console;

fn main() {
    let markdown_text = r#"
# Welcome
This is **\*bold _bold and italic_** text!
"#;

    if let Err(err) = parse_to_console(markdown_text) {
        println!("Error: {}", err);
    }
}
```

### 4. Customize Parsing Behavior with Specific Rules

If you need to parse only specific parts of the Markdown using custom rules defined in `grammar.pest`, use the `parse_by_rule` function.

```rust
use rins_markdown_parser::{parse_by_rule, Grammar, Rule, ErrorParse};

fn main() -> Result<(), ErrorParse> {
    let markdown_text = "## Subheading\nSome text here.";
    let pairs = parse_by_rule(Rule::heading2, markdown_text)?;

    for pair in pairs {
        println!("Parsed pair: {:?}", pair);
    }

    Ok(())
}
```

# Command Line Interface (CLI)

The `rins_markdown_parser` provides a Command Line Interface (CLI) to interact with the markdown parser. You can use it to parse markdown files or text into HTML or view project credits.

## Installation

If you have cloned the project, build it using Cargo:

```bash
$ cargo build --release
```

This will create an executable in the `target/release` directory. Alternatively, if you installed it as a **binary** crate, you can directly use `rins_markdown_parser`.

## Commands Overview

To see the available commands, use the `--help` option or `help` subcommand:

```bash
$ rins_markdown_parser --help
```
**Output:**
```text
rins_markdown_parser vX.X.X
Allows to interact with markdown parser via a Command Line Interface.

Usage: rins_markdown_parser [COMMAND]

Commands:
  parse    Parses provided markdown text and returns it in html format
  credits  Displays credits and project information
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Commands

1. `parse`

The `parse` command is used to convert Markdown text to HTML. It accepts input either from a file or directly as text.

**Options**
* `-I, --in <input_file>`

Specifies the location of the input markdown file.

* `-O, --out <output_file>`

Specifies the location where the HTML output will be saved. If not provided, the result is printed to the console.

* `-t, --text <markdown_text>`

Accepts Markdown text directly from the CLI.
> [!NOTE]
> This option conflicts with --in and --out.

**Examples**
1. Parse a Markdown file and save the output to an HTML file:
```bash
$ rins_markdown_parser parse --in example.md --out example.html
```

2. Parse Markdown text directly from the CLI:
```bash
$ rins_markdown_parser parse --text "# Hello World\nThis is **Markdown**."
```

2. `credits`

Displays project information and credits.

```bash
$ rins_markdown_parser credits
```

3. `help [COMMAND]`

Displays helpful information about available subcommands and their arguments.

---

### License and Usage

This project is intended solely for personal and educational use. It was never intented to be used at production. Use with caution.

### Credits

- Author: [r-rin](https://github.com/r-rin)
- This parser was developed as part of the Rust Programming Language course at NaUKMA with the support of the Ukrainian Rust community.

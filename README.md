# Simple Markdown Parser
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

## Installation
soon.

## Usage
soon.

## Examples
soon.
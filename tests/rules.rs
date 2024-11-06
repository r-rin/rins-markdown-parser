use rins_markdown_parser::*;

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
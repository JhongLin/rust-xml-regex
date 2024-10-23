use regex::Regex;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<_>>();
    let default_input: String = "".to_string();

    // 0. The get index is changed to 1 and the input will be trimmed leading/trailing spaces.
    let input: &str = args.get(1).unwrap_or(&default_input).trim();

    let result: &str = if determine_xml(input) {
        "Valid"
    } else {
        "Invalid"
    };

    println!("{}", result);
}

/// Print `Valid` if the input is a valid XML string or `Invalid` otherwise.
/// 
/// # Examples
/// 
/// ```
/// assert_eq!(true, "<Design><Code>hello world</Code></Design>");
/// assert_eq!(false, "<Design><Code>hello world</Code></Design><People>");
/// ```
/// 
/// # Time complexity
/// 
/// Takes *O*(n) time where `n` is the length of the input string.
fn determine_xml(input: &str) -> bool {
    // 1. The empty input is a invalid XML
    if input == "" {
        return false;
    }

    let mut remaining: &str = input;
    let mut interval_content: &str;
    let mut after_prolog: bool = false;
    let mut after_head: bool = false;
    let mut tag_stack: Vec<&str> = Vec::new();

    let re_tag: Regex = Regex::new(r"<[\s\S]*?>").unwrap();
    let re_prolog: Regex = Regex::new(r"<\?[\s\S]*?\?>").unwrap();
    let re_comment_front: Regex = Regex::new(r"^<!--").unwrap();
    let re_comment_rear: Regex = Regex::new(r"[\s\S]*?-->").unwrap();
    let re_comment_full: Regex = Regex::new(r"^<!--[\s\S]*?-->").unwrap();
    let re_and_sign: Regex = Regex::new(r"&[^;\s]*;?").unwrap();

    // 2. Keep slicing the input string when continuously matching the XML tag
    while !remaining.is_empty() {
        // 3. Parse the XML tag
        if let Some(mat) = re_tag.find(remaining) {
            interval_content = &remaining[..mat.start()];

            // 4. Validate the values of XML tags
            if interval_content != "" {
                if !re_and_sign.find_iter(interval_content).all(|and_sign_mat| {
                    matches!(and_sign_mat.as_str(), "&lt;" | "&gt;" | "&amp;" | "&apos;" | "&quot;")
                }) {
                    return false;
                }
            }

            // 5. Ignore the part which has been validated
            remaining = &remaining[mat.end()..];

            // 6. Obtain the XML tag
            let mat_str: &str = mat.as_str();

            // 7. Check if there is a prolog in the first line of input string
            if !after_prolog {
                after_prolog = true;
                if re_prolog.is_match(mat.as_str()) {
                    remaining = remaining.trim();
                    if remaining.is_empty() {
                        return false;
                    }
                    continue;
                }
            }

            // 8. Check if any values outside the tag scope
            if !after_head {
                if mat.start() != 0 {
                    return false;
                }
                after_head = true;
            }

            // 9. Handling the comment tag
            if re_comment_front.is_match(mat_str) {
                if re_comment_full.is_match(mat_str) {
                    continue;
                }
                if let Some(comment_mat) = re_comment_rear.find(remaining) {
                    remaining = &remaining[comment_mat.end()..];
                } else {
                    return false;
                }
            }

            // 10. Check if there is a illegal char `<` in the tag name
            if mat_str[1..(mat_str.len() - 1)].contains("<") {
                return false;
            }

            // 11. Check the mapping relationship between opening tag and closing tag
            if &mat_str[..2] == "</" {
                if tag_stack.is_empty() || tag_stack.pop().unwrap() != &mat_str[2..(mat_str.len() - 1)] {
                    return false;
                }
                continue;
            }

            // 12. Record a XML tag in the stack-like `Vec<&str>`
            tag_stack.push(&mat_str[1..(mat_str.len() - 1)]);
        } else {
            // Unable to find a XML tag, break this while loop
            break;
        }
    }

    // 13. The XML is valid if and only if no left unparsing input string and all tags in the `tag_stack` got corresponding closing tags
    remaining.is_empty() && tag_stack.is_empty()
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::determine_xml;

    // You can use here to test, feel free to modify/add the test cases here.
    // You can run tests with `cargo test`.
    // You can also use other ways to test if you want.

    #[test_case("<Design><Code>hello world</Code></Design>", true ; "normal case")]
    #[test_case("<Design><Code>hello world</Code></Design><People>", false ; "no closing tag")]
    #[test_case("<People><Design><Code>hello world</People></Code></Design>", false ; "non-corresponding tags")]
    // there is no closing tag for "People age=”1”" and no opening tag for "/People"
    #[test_case("<People age=”1”>hello world</People>", false ; "attribute is not supported")]
    // The customized test cases below with test description
    #[test_case("<?xml version=\"1.0\" encoding=\"UTF-8\"?><note><to>Tove</to><from>Jani</from> <heading>Reminder</heading><body>Don't forget me this weekend!</body></note>", true ; "W3C example 1")]
    #[test_case("<?xml version=\"1.0\" encoding=\"UTF-8\"?><note><to>Tove</to><from>Jani</from><heading>Reminder</pheading><body>Don't forget me this weekend!</body></note>", false ; "W3C example 2")]
    #[test_case("<?xml version=\"1.0\" encoding=\"UTF-8\"?>", false ; "W3C example 3")]
    #[test_case("<p>This is a paragraph.</p><br />", false ; "W3C example 4")]
    #[test_case("<b><i>This text is bold and italic</b></i>", false ; "W3C example 5")]
    #[test_case("<b><i>This text is bold and italic</i></b>", true ; "W3C example 6")]
    #[test_case("<message>salary < 1000</message>", false ; "W3C example 7")]
    #[test_case("<message>salary > 1000</message>", true ; "W3C example 7a")]
    #[test_case("<message>salary &lt; 1000</message>", true ; "W3C example 8")]
    #[test_case("<message>salary & 1000</message>", false ; "W3C example 8a")]
    #[test_case("<message>salary ' \"  1000</message>", true ; "W3C example 8b")]
    #[test_case("<message>salary &lt; & 1000</message>", false ; "W3C example 8c")]
    #[test_case("<?xml version=\"1.0\" encoding=\"UTF-8\"?><note><to>To<!-- czxc & < -->ve</to><from>Jani</from> <heading>Reminder</heading><body>Don't forget me this weekend!</body></note>", true ; "W3C example 1 with comment")]
    #[test_case("<?xml version=\"1.0\" encoding=\"UTF-8\"?><note<!-- czxc & < -->><to>Tove</to><from>Jani</from> <heading>Reminder</heading><body>Don't forget me this weekend!</body></note>", false ; "W3C example 1 with wrong comment")]
    #[test_case("    <?xml version=\"1.0\" encoding=\"UTF-8\"?><note><to>Tove</to><from>Jani</from> <heading>Reminder</heading><body>Don't forget me this weekend!</body></note>", true ; "W3C example 1 with leading spaces")]
    #[test_case("<?xml version=\"1.0\" encoding=\"UTF-8\"?><note><to>Tove</to><from>Jani</from> <heading>Reminder</heading><body>Don't forget me this weekend!</body></note>    ", true ; "W3C example 1 with trailing spaces")]
    #[test_case("<?xml version=\"1.0\" encoding=\"UTF-8\"?>    <note><to>Tove</to><from>Jani</from> <heading>Reminder</heading><body>Don't forget me this weekend!</body></note>", true ; "W3C example 1 with spaces in front of first tag")]
    #[test_case("", false ; "Empty string")]
    fn check_determine_xml(input: &'static str, expected: bool) {
        let input: String = input.to_string();
        assert_eq!(expected, determine_xml(&input));
    }
}

# Validation of XML string

The small practice from BoostDraft.

## Requirements

- The string will be passed as a parameter from the console command.
    - e.g. `./the_executable "The-parameter-string"
- The program needs to check if the input is a valid XML string, and print the result `Valid` or `Invalid` on the console.
- To simplify the question, the attributes of the XML tag are considered to the tag itself.
- This code should have good expression so that the other developers can use and extend this code.

## Consideration

- What's exactly a VALID XML?
    - The definition from [IBM](https://www.ibm.com/docs/en/b2bis?topic=syntax-xml-rules) and [W3C](https://www.w3schools.com/xml/xml_syntax.asp)
        - All XML elements must have a pair of opening and closing tag.
        - XML tags are case sensitive.
        - All XML elements must be properly nested.
        - All XML documents must have a root element.
            - But the first element may be the `prolog`.
                - `<?xml version="1.0" encoding="UTF-8"?>`
        - Attribute values must always be quoted.
        - Comments in XML
            - `<!-- This is a comment -->`
        - There are `5` pre-defined entity references in XML:
            - `&lt;` `<`
            - `&gt;` `>`
            - `&amp;` `&`
            - `&apos;` `'`
            - `&quot;` `"`
        - > Only `<` and `&` are strictly illegal in XML, but it is a good habit to replace > with `&gt;` as well.
    - Also, the leading/trailing spaces are valid in the input string or the first tag after the prolog tag.

## Method

- Use regex to parse the input
    - Utilize the crate `regex = "1.10.6"`

- Use stack structure to validate the result
    - Implement on `Vec<&str>`

## Logic flow

0. Trim the leading/trailing spaces from the input
1. Determine the input as `Invalid` if it is empty string
2. Traverse the input string and obtain the XML tags from the front
    - Validate the value of the tags
    - Validate the tag name including the comment tag
    - Match the corresponding opening/closing tags

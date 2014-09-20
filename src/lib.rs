#![feature(phase)]
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

use std::io::{File, MemWriter, stdout};

// Helpers

// Retrieves a file from disk and converts to string. Returns the template string.
fn get_template(template_path: &str) -> Result<String, int> {
    let path = Path::new(template_path);
    let display = path.display();

    // open the file
    let mut file = match File::open(&path) {
        Err(why) => fail!("Couldn't open {}: {}", display, why.desc),
        Ok(file) => file,
    };

    // read file to string 
    let template_str: String = match file.read_to_string() {
        Err(why) => fail!("couldn't read {}: {}", display, why.desc),
        Ok(string) =>  string,
    };

    Ok(template_str)
}


fn render_template_with_data<W: Writer>(writer: &mut W, data: &str) {
    writer.write_str(data).unwrap();
}

// TODO: find out how to get around the limitation of traits in test-function signatures.
// this does not work.
#[test]
// fn should_render_template() {
//     let fake_template:&str = "<div>";
//     render_test_helper(fake_template);
// }


#[test]
fn should_retrieve_file() {

    let path = "src/test_templates/sample.html";
    let expected = String::from_str("<div>");
    let retrieved_template = get_template(path);

    // for testing a stream - not working yet.
    // let passed_template: &str = retrieved_template.as_slice();
    // render_template_with_data(&stream, passed_template);

    assert_eq!(retrieved_template, Ok(expected));
}


// Capture all regex matches for rustache tags and return them as a vector of
// string slices.  Results will be used by the parser in order to create the
// TagMap.
fn find_tag_matches(input: &str) -> Vec<&str>{
    let mut result: Vec<&str> = Vec::new();
    let re = regex!(r"(\{\{\s?[\w\s]*\s?\}\})");
    
    for cap in re.captures_iter(input) {
        result.push(cap.at(1));
    }

    result
}



#[test]
fn test_bucketing() {
    let test_string: &str = "{{variable1}},{{variable2}},{{variable3}}";
    let expected: Vec<&str> = vec!["{{variable1}}","{{variable2}}","{{variable3}}"];
    let result = find_tag_matches(test_string);
    assert_eq!(result, expected);
}
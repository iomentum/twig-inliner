use std::fs;

fn main() {
    let file_contents = get_file("test.html");

    let file_contents = clean_string(file_contents);

    fs::write("result.html", file_contents);
}

fn get_file(filename: &str) -> String {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    contents
}

fn clean_string(content: String) -> String {
    let mut content = content;

    (1..100).for_each(|_| {
        content = content
            .replace("\n", "")
            .replace("> ", ">")
            .replace(" <", "<")
            .replace(" /", "/")
            .replace("; ", ";")
            .replace("{ ", "{")
            .replace(" {", "{")
            .replace(" }", "}")
            .replace("} ", "}")
            .replace(": ", ":")
            .replace("\" ", "\"")
            .replace("  ", " ");
    });

    content
}

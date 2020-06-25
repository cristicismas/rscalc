pub mod parser;

use text_io::read;
use parser::parse_line_to_elements;

fn read_line() -> String {
    let line: String = read!("{}\n");

    return line;
}

fn main() {
    loop {
        let line = read_line();
        let elements = parse_line_to_elements(line);

        for i in 0..elements.len() {
            println!("{}", elements[i]);
        }
    }
}

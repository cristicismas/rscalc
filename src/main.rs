pub mod parser;
pub mod calculator;

use text_io::read;
use parser::parse_line_to_elements;
use calculator::calculate;

static SYMBOLS: &'static [&str] = &["+", "-", "/", "*", "%"];

fn read_line() -> String {
    let line: String = read!("{}\n");

    return line;
}

fn main() {
    loop {
        let line = read_line();
        let elements = parse_line_to_elements(line);

        let result = match calculate(elements) {
            Ok(value) => value,
            Err(_) => String::from("Operation failed. Please make sure your input is correct."),
        };

        println!("{}", result);
    }
}

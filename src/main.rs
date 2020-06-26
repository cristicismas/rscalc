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

        let result = calculate(elements);

        println!("{}", result);
    }
}

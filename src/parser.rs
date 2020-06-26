use super::SYMBOLS;

fn remove_whitespace(string: &str) -> String {
  string.chars().filter(|character| !character.is_whitespace()).collect()
}

fn push_accumulator(vector: &mut Vec<String>, accumulator: &mut String) {
  let copy_of_accumulator = accumulator.clone();
  vector.push(copy_of_accumulator);

  accumulator.clear();
}

pub fn parse_line_to_elements(line: String) -> Vec<String> {
  let trimmed_line = remove_whitespace(&line);
  
  let mut elements: Vec<String> = Vec::new();

  let mut accumulator = String::new();

  for index in 0..trimmed_line.chars().count() {
    let current_char = trimmed_line.chars().nth(index).unwrap();

    if SYMBOLS.contains(&&current_char.to_string()[..]) {
      push_accumulator(&mut elements, &mut accumulator);
      accumulator.push(current_char);
      push_accumulator(&mut elements, &mut accumulator);
    } else {
      accumulator.push(current_char);
    }
  }

  let copy_of_accumulator = accumulator.clone();
  elements.push(copy_of_accumulator);

  accumulator.clear();

  return elements;
}

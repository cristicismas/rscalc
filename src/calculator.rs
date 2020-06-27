use super::SYMBOLS;
use std::num::ParseFloatError;

pub fn calculate(elements: Vec<String>) -> Result<String, ParseFloatError> {
  let mut result: f64 = elements[0].parse::<f64>()?;

  for i in 0..elements.len() {
    let current_element = &elements[i][..];

    if SYMBOLS.contains(&current_element) {
      let next_element = elements[i + 1].parse::<f64>()?;

      match current_element {
        "+" => result += next_element,
        "-" => result -= next_element,
        "/" => result /= next_element,
        "*" => result *= next_element,
        "%" => result %= next_element,
        _ => {}
      }
    }
  }

  return Ok(result.to_string());
}

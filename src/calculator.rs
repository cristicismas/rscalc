use super::SYMBOLS;
use std::num::ParseFloatError;

pub fn calculate(elements: Vec<String>) -> Result<String, ParseFloatError> {
  let mut result: f64 = elements[0].parse::<f64>()?;

  for i in 0..elements.len() {
    let current_element = &elements[i][..];

    if SYMBOLS.contains(&current_element) {
      match current_element {
        "+" => result += elements[i + 1].parse::<f64>()?,
        "-" => result -= elements[i + 1].parse::<f64>()?,
        "/" => result /= elements[i + 1].parse::<f64>()?,
        "*" => result *= elements[i + 1].parse::<f64>()?,
        "%" => result %= elements[i + 1].parse::<f64>()?,
        _ => {}
      }
    }
  }

  return Ok(result.to_string());
}

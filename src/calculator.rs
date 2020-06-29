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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_addition() {
    let elements = vec![String::from("4"), String::from("+"), String::from("5")];

    assert_eq!(calculate(elements), Ok(String::from("9")));
  }

  #[test]
  fn test_subtraction() {
    let elements = vec![String::from("10"), String::from("-"), String::from("2")];

    assert_eq!(calculate(elements), Ok(String::from("8")));
  }

  #[test]
  fn test_division() {
    let elements = vec![String::from("10"), String::from("/"), String::from("5")];

    assert_eq!(calculate(elements), Ok(String::from("2")));
  }

  #[test]
  fn test_multiplication() {
    let elements = vec![String::from("3"), String::from("*"), String::from("4")];

    assert_eq!(calculate(elements), Ok(String::from("12")));
  }

  #[test]
  fn test_modulo() {
    let elements = vec![String::from("15"), String::from("%"), String::from("10")];

    assert_eq!(calculate(elements), Ok(String::from("5")));
  }
}

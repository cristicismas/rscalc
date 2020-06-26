use super::SYMBOLS;

pub fn calculate(elements: Vec<String>) -> String {
  let mut result: f64 = elements[0].parse::<f64>().unwrap();

  for i in 0..elements.len() {
    let current_element = &elements[i][..];

    if SYMBOLS.contains(&current_element) {
      match current_element {
        "+" => result += elements[i + 1].parse::<f64>().unwrap(),
        "-" => result -= elements[i + 1].parse::<f64>().unwrap(),
        "/" => result /= elements[i + 1].parse::<f64>().unwrap(),
        "*" => result *= elements[i + 1].parse::<f64>().unwrap(),
        "%" => result %= elements[i + 1].parse::<f64>().unwrap(),
        _ => {}
      }
    }
  }

  return result.to_string();
}

pub fn to_css_camel_case<T: AsRef<str>>(input: T) -> String {
  let s = input.as_ref();
  let mut result = String::with_capacity(s.len());
  let mut iter = s.chars();

  if s.starts_with("-webkit") {
    result.push_str("webkit");
    iter.nth(6);
  }

  while let Some(char) = iter.next() {
    if char == '-' {
      if let Some(next) = iter.next() {
        result.push(next.to_ascii_uppercase());
      }
    } else {
      result.push(char);
    }
  }

  result
}

pub fn to_css_kebab_case<T: AsRef<str>>(input: T) -> String {
  let s = input.as_ref();
  let mut result = String::with_capacity(s.len() + 5);
  let mut iter = s.chars();

  if s.starts_with("webkit") {
    result.push_str("-webkit");
    iter.nth(5);
  }

  for char in iter {
    if char.is_ascii_uppercase() {
      result.push('-');
      result.push(char.to_ascii_lowercase());
    } else {
      result.push(char);
    }
  }

  result
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_to_css_camel_case() {
    assert_eq!(to_css_camel_case("foo"), "foo");
    assert_eq!(to_css_camel_case("foo-bar"), "fooBar");
    assert_eq!(to_css_camel_case("foo-bar-baz"), "fooBarBaz");
    assert_eq!(to_css_camel_case("foo-bar-baz-qux"), "fooBarBazQux");
    assert_eq!(to_css_camel_case("-webkit-foo-bar"), "webkitFooBar");
  }

  #[test]
  fn test_to_kebab_case() {
    assert_eq!(to_css_kebab_case("foo"), "foo");
    assert_eq!(to_css_kebab_case("fooBar"), "foo-bar");
    assert_eq!(to_css_kebab_case("fooBarBaz"), "foo-bar-baz");
    assert_eq!(to_css_kebab_case("fooBarBazQux"), "foo-bar-baz-qux");
    assert_eq!(to_css_kebab_case("webkitFooBar"), "-webkit-foo-bar");
  }
}

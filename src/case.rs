pub fn to_css_kebab_case<T: AsRef<str>>(input: T) -> String {
    let input = input.as_ref();

    let result_length =
        input.chars().fold(0, |acc, c| {
            acc + c.is_uppercase().then(|| 1).unwrap_or(0) + 1
        }) + input.starts_with("webkit").then(|| 1).unwrap_or_default();

    let mut result = String::with_capacity(result_length);

    if input.starts_with("webkit") {
        result.push('-');
    }

    for c in input.chars() {
        if c.is_uppercase() {
            result.push('-');
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c);
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_kebab_case() {
        assert_eq!(to_css_kebab_case("foo"), "foo");
        assert_eq!(to_css_kebab_case("fooBar"), "foo-bar");
        assert_eq!(to_css_kebab_case("fooBarBaz"), "foo-bar-baz");
        assert_eq!(to_css_kebab_case("fooBarBazQux"), "foo-bar-baz-qux");
        assert_eq!(to_css_kebab_case("webkitFooBar"), "-webkit-foo-bar");
    }
}

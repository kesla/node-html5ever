use regex::Regex;

lazy_static! {
    static ref CAMEL: Regex = Regex::new(r"(^webkit)?([A-Z])").unwrap();
}

fn replacer(cap: &regex::Captures) -> String {
    format!(
        "{}-{}",
        cap.get(1).is_some().then(|| "-webkit").unwrap_or_default(),
        cap.get(2).unwrap().as_str().to_lowercase()
    )
}

pub fn to_css_kebab_case<T: AsRef<str>>(input: T) -> String {
    CAMEL.replace_all(input.as_ref(), replacer).to_string()
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

use cssparser::{serialize_identifier, CowRcStr, ParseError, SourceLocation, ToCss};
use selectors::{
  matching::{matches_selector, MatchingContext, MatchingMode, QuirksMode},
  parser::{NonTSPseudoClass, SelectorParseErrorKind},
  SelectorImpl, SelectorList,
};
use std::{fmt::Write, ops::Deref};

use crate::ElementRef;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct AttrValue(String);

impl ToCss for AttrValue {
  fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
  where
    W: std::fmt::Write,
  {
    write!(cssparser::CssStringWriter::new(dest), "{}", &self.0)
  }
}

impl<'a> From<&'a str> for AttrValue {
  fn from(value: &'a str) -> Self {
    Self(value.into())
  }
}

impl AsRef<str> for AttrValue {
  fn as_ref(&self) -> &str {
    &self.0
  }
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct StringValue(String);

impl ToString for StringValue {
  fn to_string(&self) -> String {
    self.0.clone()
  }
}

impl ToCss for StringValue {
  fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
  where
    W: std::fmt::Write,
  {
    serialize_identifier(&self.0, dest)
  }
}

impl<'a> From<&'a str> for StringValue {
  fn from(value: &'a str) -> Self {
    Self(value.into())
  }
}

impl Deref for StringValue {
  type Target = String;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

#[derive(Eq, PartialEq, Clone)]
pub enum PseudoElement {
  Before,
  After,
}

impl selectors::parser::PseudoElement for PseudoElement {
  type Impl = Selectors;
}

impl ToCss for PseudoElement {
  fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
  where
    W: std::fmt::Write,
  {
    match self {
      PseudoElement::Before => dest.write_str("::before"),
      PseudoElement::After => dest.write_str("::after"),
    }
  }
}

#[derive(Eq, PartialEq, Clone)]
pub enum PseudoClass {
  Hover,
  Active,
  Lang(StringValue),
  AnyLink,
  Link,
  Visited,
  Focus,
  Enabled,
  Disabled,
  Checked,
  Indeterminate,
}

impl NonTSPseudoClass for PseudoClass {
  type Impl = Selectors;

  fn is_active_or_hover(&self) -> bool {
    matches!(self, Self::Hover | Self::Active)
  }

  fn is_user_action_state(&self) -> bool {
    self.is_active_or_hover()
  }
}

impl ToCss for PseudoClass {
  fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
  where
    W: std::fmt::Write,
  {
    match self {
      Self::Hover => dest.write_str(":hover"),
      Self::Active => dest.write_str(":active"),
      Self::Lang(lang) => {
        dest.write_str(":lang(")?;
        lang.to_css(dest)?;
        dest.write_str(")")
      }
      Self::AnyLink => dest.write_str(":any-link"),
      Self::Link => dest.write_str(":link"),
      Self::Visited => dest.write_str(":visited"),
      Self::Focus => dest.write_str(":focus"),
      Self::Enabled => dest.write_str(":enabled"),
      Self::Disabled => dest.write_str(":disabled"),
      Self::Checked => dest.write_str(":checked"),
      Self::Indeterminate => dest.write_str(":indeterminate"),
    }
  }
}

#[derive(Debug, Clone)]
pub struct Selectors(Vec<Selector>);

impl SelectorImpl for Selectors {
  type ExtraMatchingData = ();

  type AttrValue = AttrValue;

  type Identifier = StringValue;

  type LocalName = StringValue;

  type NamespaceUrl = StringValue;

  type NamespacePrefix = StringValue;

  type BorrowedNamespaceUrl = StringValue;

  type BorrowedLocalName = StringValue;

  type NonTSPseudoClass = PseudoClass;

  type PseudoElement = PseudoElement;
}

#[derive(Debug, Clone)]
pub struct Selector(selectors::parser::Selector<Selectors>);

struct Parser;

impl<'i> selectors::parser::Parser<'i> for Parser {
  type Impl = Selectors;
  type Error = SelectorParseErrorKind<'i>;

  fn parse_non_ts_pseudo_class(
    &self,
    location: SourceLocation,
    name: CowRcStr<'i>,
  ) -> Result<PseudoClass, ParseError<'i, SelectorParseErrorKind<'i>>> {
    use self::PseudoClass::*;
    if name.eq_ignore_ascii_case("any-link") {
      Ok(AnyLink)
    } else if name.eq_ignore_ascii_case("link") {
      Ok(Link)
    } else if name.eq_ignore_ascii_case("visited") {
      Ok(Visited)
    } else if name.eq_ignore_ascii_case("active") {
      Ok(Active)
    } else if name.eq_ignore_ascii_case("focus") {
      Ok(Focus)
    } else if name.eq_ignore_ascii_case("hover") {
      Ok(Hover)
    } else if name.eq_ignore_ascii_case("enabled") {
      Ok(Enabled)
    } else if name.eq_ignore_ascii_case("disabled") {
      Ok(Disabled)
    } else if name.eq_ignore_ascii_case("checked") {
      Ok(Checked)
    } else if name.eq_ignore_ascii_case("indeterminate") {
      Ok(Indeterminate)
    } else {
      Err(
        location.new_custom_error(SelectorParseErrorKind::UnsupportedPseudoClassOrElement(
          name,
        )),
      )
    }
  }
}

impl Selectors {
  pub fn compile(css: String) -> napi::Result<Selectors> {
    let mut parser = cssparser::ParserInput::new(css.as_str());
    match SelectorList::parse(&Parser, &mut cssparser::Parser::new(&mut parser)) {
      Ok(list) => Ok(Selectors(list.0.into_iter().map(Selector).collect())),
      Err(e) => {
        let reason = format!("Failed to parse selector: {:?}", e);
        Err(napi::Error::from_reason(reason))
      }
    }
  }

  pub fn matches(&self, element: &ElementRef) -> bool {
    self.0.iter().any(|selector| selector.matches(element))
  }
}

impl Selector {
  pub fn matches(&self, element: &ElementRef) -> bool {
    let mut context = MatchingContext::new(MatchingMode::Normal, None, None, QuirksMode::NoQuirks);

    matches_selector(&self.0, 0, None, element, &mut context, &mut |_, _| {})
  }
}

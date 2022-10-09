use cssparser::{serialize_identifier, ToCss};
use selectors::{parser::NonTSPseudoClass, SelectorImpl};
use std::{fmt::Write, ops::Deref};

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
    }
  }
}

#[derive(Debug, Clone)]
pub struct Selectors;

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

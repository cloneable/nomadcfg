use jrsonnet_evaluator::typed::ValType;
use serde::{de, ser};
use std::{cell::RefCell, fmt, rc::Rc};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub struct Suggestion(pub Option<&'static str>);

impl fmt::Display for Suggestion {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self.0 {
      Some(s) => write!(f, "{s}"),
      None => write!(f, "none"),
    }
  }
}

impl From<Option<&'static str>> for Suggestion {
  fn from(value: Option<&'static str>) -> Self {
    Suggestion(value)
  }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("{0}: field not visible: {1}")]
  FieldNotVisible(ValPath, String),

  #[error("{0}: field not found: {1}")]
  FieldNotFound(ValPath, String),

  #[error("{0}: field not expected: {1} (suggestion: {2})")]
  FieldNotExpected(ValPath, String, Suggestion),

  #[error("{path}: unexpected type: {actual}, want {expected}")]
  UnexpectedType { path: ValPath, expected: ValType, actual: ValType },

  #[error("{0}: expected identifier")]
  IdentifierExpected(ValPath),

  #[error("{0}: unexpected value: {1}")]
  UnexpectedVal(ValPath, ValType),

  #[error("serde error: {0}")]
  Serde(String),

  #[error("evaluator error: {0}")]
  Evaluator(#[from] jrsonnet_evaluator::Error),

  #[error("{0}: invalid key field value: {1}")]
  InvalidKeyFieldValue(ValPath, ValType),

  #[error("{0}: unimplemented: {1}")]
  Unimplemented(ValPath, String),
}

impl ser::Error for Error {
  fn custom<T: fmt::Display>(msg: T) -> Self {
    Error::Serde(msg.to_string())
  }
}

impl de::Error for Error {
  fn custom<T: fmt::Display>(msg: T) -> Self {
    Error::Serde(msg.to_string())
  }
}

#[derive(Debug, Clone, Default)]
pub struct RcValPath(Rc<RefCell<ValPath>>);

impl RcValPath {
  pub fn push(&mut self, entry: ValPathEntry) -> RcValPathGuard {
    self.0.borrow_mut().0.push(entry);
    RcValPathGuard { path: self.clone() }
  }

  fn pop(&mut self) -> &mut Self {
    self.0.borrow_mut().0.pop().expect("path empty");
    self
  }

  pub fn entries(&self) -> ValPath {
    self.0.borrow().clone()
  }
}

pub struct RcValPathGuard {
  path: RcValPath,
}

impl Drop for RcValPathGuard {
  fn drop(&mut self) {
    self.path.pop();
  }
}

#[derive(Debug, Clone, Default)]
pub struct ValPath(Vec<ValPathEntry>);

impl fmt::Display for ValPath {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for entry in &self.0 {
      entry.fmt(f)?;
    }
    Ok(())
  }
}

#[derive(Debug, Clone)]
pub enum ValPathEntry {
  Field(String),
  Index(usize),
}

impl fmt::Display for ValPathEntry {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      ValPathEntry::Index(i) => write!(f, "[{i}]"),
      ValPathEntry::Field(name) => write!(f, ".{name}"), // TODO: escaping
    }
  }
}

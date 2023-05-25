use jrsonnet_evaluator::Val;
use serde::{de, ser};
use std::{cell::RefCell, fmt, rc::Rc};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("FieldNotVisible: {0}.{1}")]
    FieldNotVisible(ValPath, String),

    #[error("FieldNotFound: {0}.{1}")]
    FieldNotFound(ValPath, String),

    #[error("{0}: expected Bool, found {1:?}")]
    ExpectedBool(ValPath, Val),

    #[error("{0}: expected Null, found {1:?}")]
    ExpectedNull(ValPath, Val),

    #[error("{0}: expected Str, found {1:?}")]
    ExpectedStr(ValPath, Val),

    #[error("{0}: expected Num, found {1:?}")]
    ExpectedNum(ValPath, Val),

    #[error("{0}: expected Array, found {1:?}")]
    ExpectedArr(ValPath, Val),

    #[error("{0}: expected Obj, found {1:?}")]
    ExpectedObj(ValPath, Val),

    #[error("{0}: expected Func, found {1:?}")]
    ExpectedFunc(ValPath, Val),

    #[error("{0}: expected identifier")]
    IdentifierExpected(ValPath),

    #[error("{0}: unexpected value: {1:?}")]
    UnexpectedVal(ValPath, Val),

    #[error("serde error: {0}")]
    Serde(String),

    #[error("evaluator error: {0}")]
    Evaluator(#[from] jrsonnet_evaluator::Error),
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

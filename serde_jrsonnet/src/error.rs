use jrsonnet_evaluator::Val;
use serde::{de, ser};
use std::{cell::RefCell, fmt, rc::Rc};

pub type Result<T> = std::result::Result<T, Error>;

// This is a bare-bones implementation. A real library would provide additional
// information in its error type, for example the line and column at which the
// error occurred, the byte offset into the input, or the current key being
// processed.
#[derive(Debug)]
pub enum Error {
    FieldNotVisible(Vec<ValPathEntry>, String),
    FieldNotFound(Vec<ValPathEntry>, String),

    ExpectedBool(Vec<ValPathEntry>, Val),
    ExpectedNull(Vec<ValPathEntry>, Val),
    ExpectedStr(Vec<ValPathEntry>, Val),
    ExpectedNum(Vec<ValPathEntry>, Val),
    ExpectedArr(Vec<ValPathEntry>, Val),
    ExpectedObj(Vec<ValPathEntry>, Val),
    ExpectedFunc(Vec<ValPathEntry>, Val),

    IdentifierExpected,
    UnexpectedVal(Vec<ValPathEntry>, Val),

    // One or more variants that can be created by data structures through the
    // `ser::Error` and `de::Error` traits. For example the Serialize impl for
    // Mutex<T> might return an error because the mutex is poisoned, or the
    // Deserialize impl for a struct may return an error because a required
    // field is missing.
    Message(String),
    Eval(jrsonnet_evaluator::Error),
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(&self, f) // TODO

        // match self {
        //     Error::Message(msg) => formatter.write_str(msg),
        //     Error::Eof => formatter.write_str("unexpected end of input"),
        //     /* and so forth */
        // }
    }
}

impl std::error::Error for Error {}

impl From<jrsonnet_evaluator::Error> for Error {
    fn from(value: jrsonnet_evaluator::Error) -> Self {
        Error::Eval(value)
    }
}

#[derive(Debug, Clone, Default)]
pub struct ValPath(Rc<RefCell<Vec<ValPathEntry>>>);

impl ValPath {
    pub fn push(&mut self, entry: ValPathEntry) -> ValPathGuard {
        self.0.borrow_mut().push(entry);
        ValPathGuard { path: self.clone() }
    }

    fn pop(&mut self) -> &mut Self {
        self.0.borrow_mut().pop().expect("path empty");
        self
    }

    pub fn entries(&self) -> Vec<ValPathEntry> {
        self.0.borrow().clone()
    }
}

pub struct ValPathGuard {
    path: ValPath,
}

impl Drop for ValPathGuard {
    fn drop(&mut self) {
        self.path.pop();
    }
}

#[derive(Debug, Clone)]
pub enum ValPathEntry {
    Field(String),
    ArrayIndex(usize),
}

use jrsonnet_evaluator::Val;
use serde::{de, ser};
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

// This is a bare-bones implementation. A real library would provide additional
// information in its error type, for example the line and column at which the
// error occurred, the byte offset into the input, or the current key being
// processed.
#[derive(Debug)]
pub enum Error {
    FieldNotVisible(String),
    FieldNotFound(String),

    ExpectedBool(Val),
    ExpectedNull(Val),
    ExpectedStr(Val),
    ExpectedNum(Val),
    ExpectedArr(Val),
    ExpectedObj(Val),
    ExpectedFunc(Val),

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

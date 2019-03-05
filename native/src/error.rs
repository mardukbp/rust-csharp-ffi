use std::fmt::{
    Debug,
    Display,
};

use failure::{
    self,
    err_msg,
    Fail,
};

#[derive(Debug, Fail)]
#[fail(display = "error using a db")]
pub struct Error(#[cause] failure::Error);

impl Error {
    pub(crate) fn from_fail(err: impl Fail) -> Self {
        Error(err.into())
    }

    pub(crate) fn msg(msg: impl Display + Debug + Sync + Send + 'static) -> Self {
        Error::from_fail(err_msg(msg).compat())
    }
}

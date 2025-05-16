use napi::{Error, Status};

pub type XWinError = Box<dyn std::error::Error>;

pub fn xwin_error(err: XWinError) -> Error {
  Error::new(Status::GenericFailure, format!("{}", err))
}

extern crate mime;
extern crate magic;

#[cfg(feature = "rocket_data")]
extern crate rocket;

use magic::{Cookie, flags, MagicError};
use std::path::Path;
use std::{error, fmt};
use mime::FromStrError;

pub struct MimeDetective {
    cookie: Cookie
}

impl MimeDetective {
    pub fn new() -> Result<MimeDetective, DetectiveError> {
        let cookie = Cookie::open(flags::MIME_TYPE)?;
        cookie.load(&["/usr/share/misc/magic.mgc"]).unwrap();
        Ok(MimeDetective {
            cookie
        })
    }

    pub fn detect_file<P: AsRef<Path>>(&self, filename: P) -> Result<mime::Mime, DetectiveError> {
        let mime_str = self.cookie.file(filename)?;
        let mime: mime::Mime = mime_str.parse()?;
        Ok(mime)
    }

    pub fn detect_buffer(&self, buffer: &[u8]) -> Result<mime::Mime, DetectiveError> {
        let mime_str = self.cookie.buffer(buffer)?;
        let mime: mime::Mime = mime_str.parse()?;
        Ok(mime)
    }

    #[cfg(feature = "rocket_data")]
    pub fn detect_data(&self, data: rocket::Data) -> Result<mime::Mime, DetectiveError> {
        let mime_str = self.cookie.buffer(data.as_slice())?;
        let mime: mime::Mime = mime_str.parse()?;
        Ok(mime)
    }
}

#[derive(Debug)]
pub enum DetectiveError {
    Magic(MagicError),
    Parse(FromStrError)
}

impl error::Error for DetectiveError {
    fn description(&self) -> &str {
        match *self {
            DetectiveError::Magic(ref err) => err.description(),
            DetectiveError::Parse(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            DetectiveError::Magic(ref err) => err.cause(),
            DetectiveError::Parse(ref err) => err.cause()
        }
    }
}

impl fmt::Display for DetectiveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DetectiveError::Magic(ref err) => write!(f, "MagicError: {}", err),
            DetectiveError::Parse(ref err) => write!(f, "MimeParseError: {}", err)
        }
    }
}

impl From<MagicError> for DetectiveError {
    fn from(err: MagicError) -> Self {
        DetectiveError::Magic(err)
    }
}

impl From<FromStrError> for DetectiveError {
    fn from(err: FromStrError) -> Self {
        DetectiveError::Parse(err)
    }
}
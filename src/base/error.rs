use std::ffi::OsString;
use std::path::PathBuf;
use std::{error, fmt, io, result, str};

use ::goblin::error as goblin;

#[derive(Debug)]
pub enum Error {
    InvalidDestination(PathBuf),
    NonEmptyDestionation(PathBuf),
    InvalidGlobPattern(String),
    SharedLibraryLookup(String),
    ResolverCompilation(String),
    MalformedExecutable(String),
    ValueNotFoundInStrtab { tag: u64, val: u64 },
    InterpretorNotFound,
    Encoding(str::Utf8Error),
    PathEncoding(OsString),
    IO(io::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidDestination(path) => {
                write!(f, "The destination is invalid: {}", path.display())
            }
            Error::NonEmptyDestionation(path) => {
                write!(f, "The destination is not empty: {}", path.display())
            }
            Error::InvalidGlobPattern(e) => write!(f, "Invalid glob pattern: {}", e),
            Error::SharedLibraryLookup(e) => write!(f, "Unable to lookup shared library: {}", e),
            Error::ResolverCompilation(e) => write!(
                f,
                "Error happend during the compilation of library resolver: {}",
                e
            ),
            Error::MalformedExecutable(e) => write!(f, "The executable is malformed: {}", e),
            Error::ValueNotFoundInStrtab { tag, val } => write!(
                f,
                "The executable is malformed: Value {} with tag {} is not found on strtab",
                val, tag
            ),
            Error::InterpretorNotFound => {
                write!(f, "Could not find an interpreter for the executable")
            }
            Error::Encoding(e) => write!(f, "Encoding error: {}", e),
            Error::PathEncoding(p) => write!(
                f,
                "Unable to interpret the path as UTF-8: {}",
                p.to_string_lossy()
            ),
            Error::IO(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl error::Error for Error {}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<str::Utf8Error> for Error {
    fn from(err: str::Utf8Error) -> Self {
        Error::Encoding(err)
    }
}

impl From<goblin::Error> for Error {
    fn from(err: goblin::Error) -> Self {
        match err {
            goblin::Error::Malformed(e) => Error::MalformedExecutable(e),
            goblin::Error::BadMagic(e) => {
                Error::MalformedExecutable(format!("unknown magic number: {}", e))
            }
            goblin::Error::Scroll(e) => {
                Error::MalformedExecutable(format!("unable to read bytes: {}", e))
            }
            goblin::Error::IO(e) => Error::IO(e),
        }
    }
}

impl From<glob::PatternError> for Error {
    fn from(err: glob::PatternError) -> Self {
        Error::InvalidGlobPattern(err.msg.to_string())
    }
}
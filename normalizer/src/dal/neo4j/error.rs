use std::fmt;
use std::fmt::{Display, Debug};

use serde::{ser};

pub struct Error {
    /// This `Box` allows us to keep the size of `Error` as small as possible. A
    /// larger `Error` type was substantially slower due to all the functions
    /// that pass around `Result<T, Error>`.
    err: Box<ErrorImpl>,
}

/// Alias for a `Result` with the error type `serde_json::Error`.
pub type Result<T> = std::result::Result<T, Error>;

/// Categorizes the cause of a `Error`.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Category {
    /// The error was caused by a failure to read or write bytes on an IO
    /// stream.
    Io,

    /// The error was caused by input that was not syntactically valid JSON.
    Syntax,

    /// The error was caused by input data that was semantically incorrect.
    ///
    /// For example, JSON containing a number is semantically incorrect when the
    /// type being deserialized into holds a String.
    Data,

    /// The error was caused by prematurely reaching the end of the input data.
    ///
    /// Callers that process streaming input may be interested in retrying the
    /// deserialization once more data is available.
    Eof,
}

#[cfg(feature = "std")]
#[allow(clippy::fallible_impl_from)]
impl From<Error> for io::Error {
    fn from(j: Error) -> Self {
        if let ErrorCode::Io(err) = j.err.code {
            err
        } else {
            match j.classify() {
                Category::Io => unreachable!(),
                Category::Syntax | Category::Data => io::Error::new(io::ErrorKind::InvalidData, j),
                Category::Eof => io::Error::new(io::ErrorKind::UnexpectedEof, j),
            }
        }
    }
}

struct ErrorImpl {
    code: ErrorCode,
}

pub(crate) enum ErrorCode {
    /// Catchall for syntax error messages
    Message(Box<str>),

    /// EOF while parsing a list.
    EofWhileParsingList,

    /// EOF while parsing an object.
    EofWhileParsingObject,

    /// EOF while parsing a string.
    EofWhileParsingString,

    /// EOF while parsing a JSON value.
    EofWhileParsingValue,

    /// Expected this character to be a `':'`.
    ExpectedColon,

    /// Expected this character to be either a `','` or a `']'`.
    ExpectedListCommaOrEnd,

    /// Expected this character to be either a `','` or a `'}'`.
    ExpectedObjectCommaOrEnd,

    /// Expected to parse either a `true`, `false`, or a `null`.
    ExpectedSomeIdent,

    /// Expected this character to start a JSON value.
    ExpectedSomeValue,

    /// Invalid hex escape code.
    InvalidEscape,

    /// Invalid number.
    InvalidNumber,

    /// Number is bigger than the maximum value of its type.
    NumberOutOfRange,

    /// Invalid unicode code point.
    InvalidUnicodeCodePoint,

    /// Control character found while parsing a string.
    ControlCharacterWhileParsingString,

    /// Object key is not a string.
    KeyMustBeAString,

    /// Lone leading surrogate in hex escape.
    LoneLeadingSurrogateInHexEscape,

    /// JSON has a comma after the last value in an array or map.
    TrailingComma,

    /// JSON has non-whitespace trailing characters after the value.
    TrailingCharacters,

    /// Unexpected end of hex excape.
    UnexpectedEndOfHexEscape,

    /// Encountered nesting of JSON maps and arrays more than 128 layers deep.
    RecursionLimitExceeded,

    /// Neo4j does not support a sequence-like data type
    UnsupportedSequenceValue,
}


impl Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorCode::Message(ref msg) => f.write_str(msg),
            ErrorCode::EofWhileParsingList => f.write_str("EOF while parsing a list"),
            ErrorCode::EofWhileParsingObject => f.write_str("EOF while parsing an object"),
            ErrorCode::EofWhileParsingString => f.write_str("EOF while parsing a string"),
            ErrorCode::EofWhileParsingValue => f.write_str("EOF while parsing a value"),
            ErrorCode::ExpectedColon => f.write_str("expected `:`"),
            ErrorCode::ExpectedListCommaOrEnd => f.write_str("expected `,` or `]`"),
            ErrorCode::ExpectedObjectCommaOrEnd => f.write_str("expected `,` or `}`"),
            ErrorCode::ExpectedSomeIdent => f.write_str("expected ident"),
            ErrorCode::ExpectedSomeValue => f.write_str("expected value"),
            ErrorCode::InvalidEscape => f.write_str("invalid escape"),
            ErrorCode::InvalidNumber => f.write_str("invalid number"),
            ErrorCode::NumberOutOfRange => f.write_str("number out of range"),
            ErrorCode::InvalidUnicodeCodePoint => f.write_str("invalid unicode code point"),
            ErrorCode::ControlCharacterWhileParsingString => {
                f.write_str("control character (\\u0000-\\u001F) found while parsing a string")
            }
            ErrorCode::KeyMustBeAString => f.write_str("key must be a string"),
            ErrorCode::LoneLeadingSurrogateInHexEscape => {
                f.write_str("lone leading surrogate in hex escape")
            }
            ErrorCode::TrailingComma => f.write_str("trailing comma"),
            ErrorCode::TrailingCharacters => f.write_str("trailing characters"),
            ErrorCode::UnexpectedEndOfHexEscape => f.write_str("unexpected end of hex escape"),
            ErrorCode::RecursionLimitExceeded => f.write_str("recursion limit exceeded"),
            ErrorCode::UnsupportedSequenceValue => f.write_str("Neo4j does not support a sequence-like data type"),
        }
    }
}

impl serde::de::StdError for Error {
    #[cfg(feature = "std")]
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&*self.err, f)
    }
}

impl Display for ErrorImpl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.code
        )
        
    }
}

// Remove two layers of verbosity from the debug representation. Humans often
// end up seeing this representation because it is what unwrap() shows.
impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error({:?})",
            self.err.code.to_string(),
        )
    }
}


fn make_error(mut msg: String) -> Error {
    Error {
        err: Box::new(ErrorImpl {
            code: ErrorCode::Message(msg.into_boxed_str())
        }),
    }
}

fn starts_with_digit(slice: &str) -> bool {
    match slice.as_bytes().get(0) {
        None => false,
        Some(&byte) => byte >= b'0' && byte <= b'9',
    }
}

impl ser::Error for Error {
    #[cold]
    fn custom<T: Display>(msg: T) -> Error {
        make_error(msg.to_string())
    }
}
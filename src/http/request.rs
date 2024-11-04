use super::{method::Method, MethodError, QueryString};
use std::{
    error::Error,
    fmt::Display,
    str::{from_utf8, Utf8Error},
};

pub struct Request<'buf> {
    pub path: &'buf str,
    pub query_string: Option<QueryString<'buf>>,
    pub method: Method,
}

impl<'buf> TryFrom<&'buf Vec<u8>> for Request<'buf> {
    type Error = ParseError;

    /*
     * GET /user?id=10 HTTP/1.1
     * HEADERS \r\n
     * BODY
     */
    fn try_from(buf: &'buf Vec<u8>) -> Result<Self, Self::Error> {
        let request = from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;

        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    None
}

#[derive(Debug)]
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidMethod => "Invalid Method",
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidEncoding => "Invalid Encoding",
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}
impl Error for ParseError {}

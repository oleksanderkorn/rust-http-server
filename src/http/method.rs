use std::str::FromStr;

pub enum Method {
    Get,
    Delete,
    Post,
    Put,
    Patch,
    Head,
    Options,
    Trace,
    Connect,
}

impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::Get),
            "DELETE" => Ok(Self::Delete),
            "POST" => Ok(Self::Post),
            "PUT" => Ok(Self::Put),
            "PATCH" => Ok(Self::Patch),
            "HEAD" => Ok(Self::Head),
            "OPTIONS" => Ok(Self::Options),
            "TRACE" => Ok(Self::Trace),
            "CONNECT" => Ok(Self::Connect),
            _ => Err(MethodError),
        }
    }
}

pub struct MethodError;

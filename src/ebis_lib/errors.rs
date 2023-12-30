#[derive(Debug)]
pub enum ParseOrReqError {
    ReqError(reqwest::Error),
    ParsingError(json::Error),
}

impl From<reqwest::Error> for ParseOrReqError {
    fn from(value: reqwest::Error) -> Self {
        ParseOrReqError::ReqError(value)
    }
}

impl From<json::Error> for ParseOrReqError {
    fn from(value: json::Error) -> Self {
        ParseOrReqError::ParsingError(value)
    }
}

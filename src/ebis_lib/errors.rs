#[derive(Debug)]
pub enum ParseOrReqError {
    ReqError(reqwest::Error),
    ParsingError(json::Error),
}

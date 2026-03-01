#[derive(Debug, Clone)]
pub enum ServerRequestError {
    /// Jsonwebtoken is missing or not correct
    NoJWTValidation,
    CouldNotRetrieveData,
    CouldNotSendData,
    /// HTTP request Error mostly used when error code is sent back
    HTTPError,
}
impl ServerRequestError {
    pub fn to_error_message(&self) -> String {
        let slice = match self {
            ServerRequestError::NoJWTValidation => {
                "Could not connect to server due to missing validation!"
            }
            ServerRequestError::CouldNotRetrieveData => {
                "Could not retrieve the data from the server!"
            }
            ServerRequestError::CouldNotSendData => "Could not send the data to the server!",
            ServerRequestError::HTTPError => "The http request failed!",
        };
        slice.to_string()
    }
}

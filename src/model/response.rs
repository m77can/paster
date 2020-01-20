#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBody<T> {
    pub data: T,
}

impl<T> ResponseBody<T> {
    pub fn new(data: T) -> ResponseBody<T> {
        ResponseBody { data }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponseBody {
    pub code: String,
    pub message: String,
}

impl ErrorResponseBody {
    pub fn new(code: &str, message: &str) -> ErrorResponseBody {
        ErrorResponseBody {
            code: code.to_string(),
            message: message.to_string(),
        }
    }
}

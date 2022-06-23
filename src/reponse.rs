use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBody<T> {
    pub message: String,
    pub data: T
}

impl<T> ResponseBody<T> {
    pub fn new(mesage: &str, data: T) -> ResponseBody<T> {
        ResponseBody {
            message: mesage.to_string(),
            data
        }
    }
}
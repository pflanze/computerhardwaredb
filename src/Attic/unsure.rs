use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unsure<T> {
    value: Option<T>,
    comment: String, // optional
}
impl<T> Unsure<T> {
    pub fn new<S: ToString>(value: Option<T>, comment: S) -> Self {
        Self { value, comment: comment.to_string() }
    }
    pub fn none() -> Self {
        Self { value: None, comment: "".into() }
    }
}
impl<T> From<T> for Unsure<T> {
    fn from(value: T) -> Self {
        Self { value: Some(value), comment: "".into() }
    }
}


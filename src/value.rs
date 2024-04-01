use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value<T> {
    Some(T),
    SomeWithDoubts(T, String),
    NotApplicable,
    Missing, // means "missing information" (not "feature unavailable in the article").
}
impl<T> From<T> for Value<T> {
    fn from(value: T) -> Self {
        Self::Some(value)
    }
}

impl<T> Value<T> {
    pub fn value(&self) -> Result<&T> {
        let err = |msg| -> Result<_> {
            match std::env::var("DEBUG_VALUE") {
                Ok(_) => panic!(),
                Err(e) => match e {
                    std::env::VarError::NotPresent => bail!("{msg}"),
                    std::env::VarError::NotUnicode(_) => Err(e)?,
                }
            }
        };
        match self {
            Value::Some(v) => Ok(v),
            Value::SomeWithDoubts(v, d) => {
                println!("Warning: there are doubts: {d}"); //
                Ok(v)
            },
            Value::NotApplicable => err("value not applicable"),
            Value::Missing => err("value missing"),
        }
    }
}

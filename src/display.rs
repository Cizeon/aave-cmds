use crate::prelude::*;

pub trait MyDisplay {
    fn to_text(&self) -> Result<String>;
    fn to_json(&self) -> Result<String>;
}

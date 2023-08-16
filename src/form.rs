use crate::model::Uint;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateForm {
    pub num: Uint,
}

#[derive(Deserialize)]
pub struct CreateUnsignedForm {
    pub num: u32,
}

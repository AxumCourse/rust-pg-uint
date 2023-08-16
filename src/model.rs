use serde::{Deserialize, Serialize};

#[derive(sqlx::Type, Clone, Copy, Deserialize, Serialize, Debug, Default)]
#[sqlx(transparent)]
#[sqlx(type_name = "uint")]
pub struct Uint(i32);

impl Uint {
    pub fn new(v: i32) -> Self {
        Self(v)
    }
    pub fn value(self) -> i32 {
        self.0
    }
    pub fn usinged(self) -> u32 {
        self.0 as u32
    }
}

impl From<i32> for Uint {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl Into<i32> for Uint {
    fn into(self) -> i32 {
        self.value()
    }
}

impl From<u32> for Uint {
    fn from(value: u32) -> Self {
        Self(value as i32)
    }
}

impl Into<u32> for Uint {
    fn into(self) -> u32 {
        self.usinged()
    }
}

#[derive(sqlx::FromRow, Clone, Deserialize, Serialize, Debug, Default)]
pub struct Test {
    pub id: String,
    pub num: Uint,
}

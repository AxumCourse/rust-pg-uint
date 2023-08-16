use serde::{Deserialize, Serialize, Serializer};

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
    #[serde(serialize_with = "uint_serialize")]
    pub num: Uint,
}

#[derive(Serialize)]
pub struct TestResp {
    pub id: String,
    pub num: Uint,
    #[serde(serialize_with = "uint_serialize")]
    pub unsigned: Uint,
}

impl From<Test> for TestResp {
    fn from(t: Test) -> Self {
        Self {
            id: t.id,
            num: t.num,
            unsigned: t.num,
        }
    }
}

impl Into<Test> for TestResp {
    fn into(self) -> Test {
        Test {
            id: self.id,
            num: self.num,
        }
    }
}


fn uint_serialize<S>(i: &Uint, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_u32(i.usinged())
}

#[derive(Debug, Serialize)]
pub struct Resp<'a, T: Serialize> {
    code: i32,
    msg: &'a str,
    data: T,
}

impl<'a, T: Serialize> Resp<'a, T> {
    pub fn new(code: i32, msg: &'a str, data: T) -> Self {
        Self { code, msg, data }
    }
    pub fn ok(data: T) -> Self {
        Self::new(0, "OK", data)
    }
}

impl<'a> Resp<'a, ()> {
    pub fn err_with_code(code: i32, msg: &'a str) -> Self {
        Self::new(code, msg, ())
    }
    pub fn err(msg: &'a str) -> Self {
        Self::err_with_code(-1, msg)
    }
}

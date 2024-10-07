#[derive(Debug)]
pub struct OtherCustomLockType;

pub type OtherCustomLockTypeBis = OtherCustomLockType;

pub fn new_bis() -> OtherCustomLockTypeBis {
    OtherCustomLockTypeBis {}
}

pub fn try_new_bis() -> Result<OtherCustomLockTypeBis, Box<dyn std::error::Error>> {
    Ok(OtherCustomLockTypeBis {})
}

pub struct Ctx {}
impl Ctx {
    pub async fn db_conn(&self) -> Result<OtherCustomLockTypeBis, Box<dyn std::error::Error>> {
        Ok(OtherCustomLockTypeBis {})
    }
}

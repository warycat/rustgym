#[derive(Debug, Queryable)]
pub struct RomData {
    pub romID: Option<i32>,
    pub releaseTitleName: Option<String>,
    pub releaseDescription: Option<String>,
    pub romHashMD5: Option<String>,
    pub romFileName: Option<String>,
    pub releaseCoverFront: Option<String>,
    pub romSize: Option<i32>,
}

// リソースファイルデータ
#[derive(Debug, Clone)]
pub struct DxResouceData {
    pub path: String,
    pub data: i32,
    pub data_handle: Vec<i32>,
}
pub trait DxResouce: Sized {
    fn open_from_file(&mut self, path: &str) -> Result<Self, u32>;
}

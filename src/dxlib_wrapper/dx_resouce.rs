// リソース関係
pub trait DxResouce {
    type Config; // 設定
    type GetVal; // ゲッターでの戻り値の型
    fn create(&mut self, config: &Self::Config) -> Result<&mut Self, String>;
    fn get(&self) -> Result<Self::GetVal,String>;
    fn delete(&mut self) -> Result<&mut Self, String>;
}
// リソースの管理
pub trait DxResouceMgr<T:DxResouce>  {
    type MgrObj; // 管理対象のオブジェクト
}

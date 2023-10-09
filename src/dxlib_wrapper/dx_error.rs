pub struct DxResouceError{
}
impl DxResouceError{
}
pub struct DxFileError{
}
impl DxFileError{
}
pub struct DxError{
}
impl DxError{
}
pub enum DxErrorType{
    ResouceE(DxResouceError),
    FileE(DxFileError),
    DxLibE(DxError),
}


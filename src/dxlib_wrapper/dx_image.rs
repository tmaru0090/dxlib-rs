pub struct DxImageData {
    image_handle: i32,
    image_path: String,
    image_size_x: i32,
    image_size_y: i32,
}
impl DxImageData {
    pub fn new() -> DxImageData {
        return DxImageData {
            image_handle: 0,
            image_path: String::new(),
            image_size_x: 0,
            image_size_y: 0,
        };
    }
}
struct DxImage {
    data: DxImageData,
}
impl DxImage {
    pub fn new(image_path: &str) -> DxImage {
        return DxImage {
            data: DxImageData::new(),
        };
    }
    pub fn create_image(&mut self) -> Result<&mut DxImage, String> {
        return Ok(self);
    }
    pub fn create_divimage(&mut self) -> Result<&mut DxImage, String> {
        return Ok(self);
    }
    pub fn delete_image(&mut self) -> Result<&mut DxImage, String> {
        return Ok(self);
    }
    pub fn delete_divimage(&mut self) -> Result<&mut DxImage, String> {
        return Ok(self);
    }
    pub fn get(&self) -> i32 {
        return self.data.image_handle;
    }
}

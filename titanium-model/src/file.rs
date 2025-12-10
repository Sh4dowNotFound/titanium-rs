/// A file to be uploaded to Discord.
#[derive(Debug, Clone)]
pub struct FileUpload {
    /// The name of the file.
    pub filename: String,
    /// The binary data of the file.
    pub data: Vec<u8>,
}

impl FileUpload {
    /// Create a new FileUpload.
    pub fn new(filename: impl Into<String>, data: impl Into<Vec<u8>>) -> Self {
        Self {
            filename: filename.into(),
            data: data.into(),
        }
    }
}

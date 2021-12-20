pub struct PartFile {
    filename: String,
    data: String,
}

pub struct MultipartFormParser {
    files: Vec<PartFile>,
}

impl<'t> FromDataSimple for MultipartFormParser {}

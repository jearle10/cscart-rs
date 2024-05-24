#[derive(Default)]
pub struct GetAllOptions {
    pub params: Option<Vec<(String, String)>>,
}

impl GetAllOptions {
    pub fn params(&self) -> &[(String, String)] {
        match &self.params {
            Some(x) => x,
            None => &[],
        }
    }
}

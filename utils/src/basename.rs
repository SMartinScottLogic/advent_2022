pub trait BaseName {
    fn base_name(&self) -> Self;
}

impl BaseName for &str {
    fn base_name(&self) -> Self {
        self.rfind('.').map_or(self, |n| &self[..n])
    }
}

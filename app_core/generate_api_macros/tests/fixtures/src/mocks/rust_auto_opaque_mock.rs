pub struct RustAutoOpaque<T> {
    model: T,
}

impl<T: std::clone::Clone> RustAutoOpaque<T> {
    fn blocking_write(&self) -> T {
        self.clone().model
    }
    fn clone(&self) -> Self {
        Self {
            model: self.model.clone(),
        }
    }
}

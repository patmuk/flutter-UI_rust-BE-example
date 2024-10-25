pub(crate) trait CqrsModel: std::marker::Sized {
    fn get_model_lock(app_state: &AppState) -> &RustAutoOpaque<Self>;
}

pub fn saves_exists(path: &str) -> bool {
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::path::Path::new(path).exists()
    }

    #[cfg(target_arch = "wasm32")]
    {
        let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
        storage.get_item(path).unwrap().is_some()
    }
}

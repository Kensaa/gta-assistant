fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        res.compile().expect("error in build script");
    }
}

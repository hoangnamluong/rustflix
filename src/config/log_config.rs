pub fn init() -> () {
    if std::env::var_os("RUST_LOG").is_none() {
        unsafe { 
            std::env::set_var("RUST_LOG", "info");
        }
    }

    env_logger::init();
}

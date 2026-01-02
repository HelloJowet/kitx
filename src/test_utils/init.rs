pub fn init_logger() {
    // Use try_init() to avoid panicking if the logger is already initialized
    // This can happen when tests are running in parallel
    let _ = env_logger::try_init();
}

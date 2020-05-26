// panic handler
rayon_core::ThreadPoolBuilder::new()
    .panic_handler(|e| {
        // logging and graceful shutdown
    })
    .num_threads(4)
    .build_global()?;


// spawn into pool
rayon_core::spawn(move || {
    sync::get_status(...)
});
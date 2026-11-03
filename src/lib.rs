#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(_app: AndroidApp) {
    android_logger::init_once(android_logger::Config::default().with_max_level(log::LevelFilter::Info));
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "FeO Launcher",
        options,
        Box::new(|_cc| Box::new(FeOLauncherApp::new())),
    ).unwrap();
}

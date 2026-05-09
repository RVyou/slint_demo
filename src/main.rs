#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use lib::ui::window::*;
use slint::SharedString;

fn main() {
    let g = slint_init();
    g.init_global();
    let w = g.main_window.as_weak();
    //异步ui和函数调用,只能main导出的变量和函数或者main导出的global
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(2));
        w.upgrade_in_event_loop(|w| {
            w.global::<AppConfig>()
                .set_app_name(SharedString::from("new name"));
        })
    });

    g.run();
}

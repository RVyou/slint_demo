#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use lib::ui::window::*;
use slint::SharedString;

fn main() {
    let g = slint_init();
    g.init_global();
    let w = g.main_window.as_weak();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(2));

        w.upgrade_in_event_loop(|w| {
            w.set_aa(SharedString::from("aaaa"));
        })
    });

    g.run();
}

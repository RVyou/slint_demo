use slint::PhysicalPosition;

slint::include_modules!();

pub fn slint_init() -> GlobalWindow {
    let main_window = MainWindow::new().unwrap();
    let global = GlobalWindow::new(main_window);
    global
}

pub struct GlobalWindow {
    pub main_window: MainWindow,
}

impl GlobalWindow {
    pub fn new(main_window: MainWindow) -> GlobalWindow {
        Self { main_window }
    }

    pub fn init_global(&self) {
        self.bing_cb_drag_window();
        self.bing_minimize();
        self.bing_maximize();
        self.bing_close();
    }

    pub fn run(&self) {
        _ = self.main_window.run();
    }

    // 自定义任务栏移动拖拽
    fn bing_cb_drag_window(&self) {
        let window_weak = self.main_window.as_weak().unwrap();
        self.main_window
            .global::<GlobalCallBack>()
            .on_drag_window(move |x, y| {
                let temp_window = window_weak.window();
                let scale = temp_window.scale_factor();
                let position = temp_window.position();
                let temp_x = position.x + (x * scale) as i32;
                let temp_y = position.y + (y * scale) as i32;
                temp_window.set_position(PhysicalPosition::new(temp_x, temp_y));
            });
    }

    // 最小化
    fn bing_minimize(&self) {
        let window_weak = self.main_window.as_weak().unwrap();
        self.main_window
            .global::<GlobalCallBack>()
            .on_minimize(move || {
                window_weak.window().set_minimized(true);
            });
    }

    // 最大化 / 恢复
    fn bing_maximize(&self) {
        let window_weak = self.main_window.as_weak().unwrap();
        self.main_window
            .global::<GlobalCallBack>()
            .on_maximize(move || {
                let w = window_weak.window();
                w.set_maximized(!w.is_maximized());
            });
    }

    // 关闭应用
    fn bing_close(&self) {
        self.main_window
            .global::<GlobalCallBack>()
            .on_close(move || {
                println!("Closing global window");
                slint::quit_event_loop().unwrap();
            });
    }
}

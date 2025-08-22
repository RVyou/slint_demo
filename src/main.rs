#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use slint::{Model, VecModel};


slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {

    let main_window = MainWindow::new()?;
    let aa = main_window.get_name();
    println!("aa: {}", aa);
    main_window.on_testaa(|tt: i32| {

        println!("testaa{}", tt);
    });
    main_window.invoke_testaa(101);
    main_window.set_structaa(TestStruct{
        I :12,
        B:"asd".into(),
    });

   let mut data =  main_window.get_name_a();
    let _a =data.as_any().downcast_ref::<VecModel<i32>>();//VecModel<i32> VecModel<SharedString>
    main_window.run()

}

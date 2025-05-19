#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use slint::Color;
use slint::SharedString;

slint::include_modules!();

fn main() {
    let main_window = AppWindow::new().unwrap();
    let main_window_weak = main_window.as_weak();
    main_window.on_exit_editor({
        move || {
            println!("Exiting editor...");
            std::process::exit(0);
        }
    });

    let canvas_color_tiles: [ColorTileData; 6]  = [
        ColorTileData { colour: Color::from_rgb_u8(255,   0,   0), index: "Some text".into() },
        ColorTileData { colour: Color::from_rgb_u8(255, 255,   0), index: "Some text".into() },
        ColorTileData { colour: Color::from_rgb_u8(255, 128, 255), index: "Some text".into() },
        ColorTileData { colour: Color::from_rgb_u8(  0,   0, 255), index: "Some text".into() },
        ColorTileData { colour: Color::from_rgb_u8(  0, 255, 255), index: "Some text".into() },
        ColorTileData { colour: Color::from_rgb_u8(255, 255, 255), index: "Some text".into() },
    ];

    let tiles_model = std::rc::Rc::new(slint::VecModel::from(canvas_color_tiles.to_vec()));
    main_window.set_canvas_color_tiles(tiles_model.clone().into());

    main_window.run().unwrap();
}

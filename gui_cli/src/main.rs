use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let use_gui = args.iter().any(|arg| arg == "--gui");

    if use_gui {
        gui_main();
    } else {
        println!("Hello World");
    }
}

async fn gui_main_async() {
    use macroquad::prelude::*;

    loop {
        clear_background(BLACK);
        
        let text = "Hello World!";
        let font_size = 40.0;
        let text_width = measure_text(text, None, font_size as u16, 1.0).width;
        
        draw_text(
            text,
            screen_width() / 2.0 - text_width / 2.0,
            screen_height() / 2.0,
            font_size,
            WHITE,
        );
        
        if is_key_down(KeyCode::Escape) {
            break;
        }
        
        next_frame().await;
    }
}

fn gui_main() {
    macroquad::Window::new("Hello World", async { gui_main_async().await });
}
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};

#[macroquad::main("buttons")]
async fn main() {
    let mut button_click_count = 0;
    let mut checkbox_checked = false;
    let mut slider_value = 50.0;
    let mut input_text = String::from("Type here...");
    let mut selected_item = 0;
    let dropdown_items = vec!["Option 1", "Option 2", "Option 3"];

    let checklist_items = vec!["Milk", "Bread", "Eggs", "Apples"];
    let mut checklist_state = vec![false; checklist_items.len()];

    loop {
        clear_background(BLACK);

        widgets::Window::new(hash!(), vec2(0.0, 0.0), vec2(screen_width(), screen_height()))
            .label("Widgets Example")
            .ui(&mut *root_ui(), |ui| {
                if ui.button(None, "Click Me!") {
                    button_click_count += 1;
                }

                ui.label(None, &format!("Clicked {} times", button_click_count));

                if ui.button(None, if checkbox_checked { "ON" } else { "OFF" }) {
                    checkbox_checked = !checkbox_checked;
                }

                if widgets::Button::new("Button2")
                    .size(vec2(150.0, 30.0))
                    .ui(ui)
                {
                    println!("Button2 clicked");
                }

                ui.separator();

                ui.slider(hash!(), "Slider", 0.0..100.0, &mut slider_value);
                ui.label(None, &format!("Value: {:.1}", slider_value));

                ui.input_text(hash!(), "Text input", &mut input_text);

                ui.combo_box(hash!(), "Choose an option", &dropdown_items, &mut selected_item);
                ui.label(None, &format!("Selected: {}", dropdown_items[selected_item]));

                ui.separator();

                ui.label(None, "Checklist:");
                for (i, item) in checklist_items.iter().enumerate() {
                    ui.checkbox(hash!(i as u64), *item, &mut checklist_state[i]);
                }
            });

        next_frame().await;
    }
}

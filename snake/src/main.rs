use macroquad::prelude::*;
use std::{thread, time};

const GRID_SIZE: usize = 10;
const CELL_SIZE: f32 = 60.0;
const GRID_SIDE: f32 = GRID_SIZE as f32 * CELL_SIZE;

struct Player {
    grid_x: usize,
    grid_y: usize,
}

impl Player {
    fn new() -> Self {
        Player {
            grid_x: 0,
            grid_y: 0,
        }
    }

    fn draw(&self) {
        let margin_w = (screen_width() - GRID_SIDE) / 2.0;
        let margin_h = (screen_height() - GRID_SIDE) / 2.0;
        let x = margin_w + self.grid_x as f32 * CELL_SIZE + CELL_SIZE / 2.0;
        let y = margin_h + self.grid_y as f32 * CELL_SIZE + CELL_SIZE / 2.0;
        draw_circle(x, y, CELL_SIZE / 3.0, BLUE);
        draw_circle(x - 5.0, y - 5.0, CELL_SIZE / 6.0, BLUE);
    }

    fn move_player(&mut self, dx: isize, dy: isize) {
        let new_x = self.grid_x as isize + dx;
        let new_y = self.grid_y as isize + dy;
        
        if new_x >= 0 && new_x < GRID_SIZE as isize {
            self.grid_x = new_x as usize;
        }
        if new_y >= 0 && new_y < GRID_SIZE as isize {
            self.grid_y = new_y as usize;
        }
    }
}

fn draw_grid() {
    let margin_w = (screen_width() - GRID_SIDE) / 2.0;
    let margin_h = (screen_height() - GRID_SIDE) / 2.0;

    draw_rectangle(
        margin_w - 5.0, 
        margin_h - 5.0, 
        GRID_SIZE as f32 * CELL_SIZE + 10.0, 
        GRID_SIZE as f32 * CELL_SIZE + 10.0, 
        Color::from_rgba(40, 40, 60, 255)
    );
    
    for i in 0..=GRID_SIZE {
        draw_line(
            margin_w + i as f32 * CELL_SIZE,
            margin_h,
            margin_w + i as f32 * CELL_SIZE,
            margin_h + GRID_SIZE as f32 * CELL_SIZE,
            2.0,
            GRAY,
        );
        
        draw_line(
            margin_w,
            margin_h + i as f32 * CELL_SIZE,
            margin_w + GRID_SIZE as f32 * CELL_SIZE,
            margin_h + i as f32 * CELL_SIZE,
            2.0,
            GRAY,
        );
    }
}

fn draw_instructions() {
    let margin_h = (screen_height() - GRID_SIDE) / 2.0;

    let text = "Use ARROW KEYS to move on the grid";
    let text_size = measure_text(text, None, 30, 1.0);
    draw_text(
        text,
        screen_width() / 2.0 - text_size.width / 2.0,
        margin_h + GRID_SIZE as f32 * CELL_SIZE + 60.0,
        30.0,
        LIGHTGRAY,
    );
}

fn draw_title(text: &str, font_size: f32, color: Color) {
    let dimensions = measure_text(text, None, font_size as u16, 1.0);
    let x = (screen_width() - dimensions.width) / 2.0;
    let y = (screen_height() - GRID_SIDE) / 4.0;
    
    draw_text(text, x, y, font_size, color);
}

#[macroquad::main("Grid Movement Game")]
async fn main() {
    let mut player = Player::new();
    
    loop {
        if is_key_down(KeyCode::Up) {
            player.move_player(0, -1);
            thread::sleep(time::Duration::from_millis(150));
        }
        if is_key_down(KeyCode::Down) {
            player.move_player(0, 1);
            thread::sleep(time::Duration::from_millis(150));
        }
        if is_key_down(KeyCode::Left) {
            player.move_player(-1, 0);
            thread::sleep(time::Duration::from_millis(150));
        }
        if is_key_down(KeyCode::Right) {
            player.move_player(1, 0);
            thread::sleep(time::Duration::from_millis(150));
        }
        
        clear_background(Color::from_rgba(20, 20, 35, 255));
        
        draw_grid();
        player.draw();
        draw_instructions();
        
        draw_title("SNAKE", 50.0, BLUE);
        
        next_frame().await
    }
}
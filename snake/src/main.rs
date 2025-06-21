use macroquad::prelude::*;
use std::{thread, time};
use std::collections::VecDeque;

const GRID_SIZE: usize = 15;
const CELL_SIZE: f32 = 40.0;
const GRID_SIDE: f32 = GRID_SIZE as f32 * CELL_SIZE;

struct Snake {
    grid_x: usize,
    grid_y: usize,
    history: VecDeque<(usize, usize)>,
}

impl Snake {
    fn new() -> Self {
        let mut history = VecDeque::new();
        history.push_back((0, 0));
        //history.push_back((0, 1));
        Snake {
            grid_x: 0,
            grid_y: 1,
            history,
        }
    }

    fn draw(&self) {
        let margin_w = (screen_width() - GRID_SIDE) / 2.0;
        let margin_h = (screen_height() - GRID_SIDE) / 2.0;
        
        for (grid_x, grid_y) in &self.history {
            let x = margin_w + *grid_x as f32 * CELL_SIZE;
            let y = margin_h + *grid_y as f32 * CELL_SIZE;
            draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, RED);
        }
    }

    fn move_snake(&mut self, dx: isize, dy: isize, award_pos: (usize, usize)) -> bool {
        let new_x = self.grid_x as isize + dx;
        let new_y = self.grid_y as isize + dy;
        
        if new_x >= 0 && new_x < GRID_SIZE as isize {
            self.grid_x = new_x as usize;
        }
        if new_y >= 0 && new_y < GRID_SIZE as isize {
            self.grid_y = new_y as usize;
        }
        if new_x >= 0 && new_x < GRID_SIZE as isize && new_y >= 0 && new_y < GRID_SIZE as isize {
            self.history.push_back((new_x as usize, new_y as usize));
        }
        if award_pos.0 == self.grid_x && award_pos.1 == self.grid_y {
            return true;
        }
        else {
            self.history.pop_front();
            return false;
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

fn place_award(snake: &Snake) -> (usize, usize) {
    let snake_pos = &snake.history;
    
    loop {
        let x = rand::gen_range(0, GRID_SIZE);
        let y = rand::gen_range(0, GRID_SIZE);
        
        if !snake_pos.contains(&(x, y)) {
            return (x, y);
        }
    }
}

fn draw_award(award_pos: (usize, usize)) {
    let margin_w = (screen_width() - GRID_SIDE) / 2.0;
    let margin_h = (screen_height() - GRID_SIDE) / 2.0;
    
    let x = margin_w + award_pos.0 as f32 * CELL_SIZE;
    let y = margin_h + award_pos.1 as f32 * CELL_SIZE;
    draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, GREEN);
}

#[macroquad::main("Snake")]
async fn main() {
    let mut snake = Snake::new();
    let mut award_pos = place_award(&snake);
    let mut score = 0;
    let mut intersects = false;
    
    loop {
        if is_key_down(KeyCode::Up) {
            intersects = snake.move_snake(0, -1, award_pos);
            thread::sleep(time::Duration::from_millis(150));
        }
        if is_key_down(KeyCode::Down) {
            intersects = snake.move_snake(0, 1, award_pos);
            thread::sleep(time::Duration::from_millis(150));
        }
        if is_key_down(KeyCode::Left) {
            intersects = snake.move_snake(-1, 0, award_pos);
            thread::sleep(time::Duration::from_millis(150));
        }
        if is_key_down(KeyCode::Right) {
            intersects = snake.move_snake(1, 0, award_pos);
            thread::sleep(time::Duration::from_millis(150));
        }
        if intersects {
            award_pos = place_award(&snake);
            score += 1;
            intersects = false;
        }
        
        clear_background(Color::from_rgba(20, 20, 35, 255));
        
        draw_grid();
        snake.draw();
        draw_instructions();
        draw_award(award_pos);
        
        draw_title("SNAKE", 50.0, BLUE);
        
        next_frame().await
    }
}
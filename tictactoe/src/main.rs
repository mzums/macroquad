use macroquad::prelude::*;

const BOARD_SIZE: usize = 3;
const CELL_SIZE: f32 = 150.0;
const BOARD_PADDING: f32 = 50.0;
const LINE_THICKNESS: f32 = 5.0;

#[derive(Clone, Copy, PartialEq)]
enum Player {
    X,
    O,
}

#[derive(Clone, Copy, PartialEq)]
enum CellState {
    Empty,
    Occupied(Player),
}

struct Game {
    board: [[CellState; BOARD_SIZE]; BOARD_SIZE],
    current_player: Player,
}

impl Game {
    fn new() -> Self {
        Self {
            board: [[CellState::Empty; BOARD_SIZE]; BOARD_SIZE],
            current_player: Player::X,
        }
    }

    fn make_move(&mut self, row: usize, col: usize) {
        if let CellState::Empty = self.board[row][col] {
            self.board[row][col] = CellState::Occupied(self.current_player);
            self.current_player = match self.current_player {
                Player::X => Player::O,
                Player::O => Player::X,
            };
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Tic Tac Toe".to_owned(),
        window_width: (CELL_SIZE * BOARD_SIZE as f32 + BOARD_PADDING * 2.0) as i32,
        window_height: (CELL_SIZE * BOARD_SIZE as f32 + BOARD_PADDING * 2.0) as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();

    loop {
        clear_background(LIGHTGRAY);

        let board_start_x = BOARD_PADDING;
        let board_start_y = BOARD_PADDING;
        let board_width = CELL_SIZE * BOARD_SIZE as f32;

        for i in 0..=BOARD_SIZE {
            let pos = board_start_x + i as f32 * CELL_SIZE;
            draw_line(pos, board_start_y, pos, board_start_y + board_width, LINE_THICKNESS, BLACK);
            draw_line(board_start_x, board_start_y + i as f32 * CELL_SIZE, 
                     board_start_x + board_width, board_start_y + i as f32 * CELL_SIZE, 
                     LINE_THICKNESS, BLACK);
        }

        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let x = board_start_x + col as f32 * CELL_SIZE;
                let y = board_start_y + row as f32 * CELL_SIZE;
                
                match game.board[row][col] {
                    CellState::Occupied(Player::X) => {
                        draw_line(x + 20.0, y + 20.0, x + CELL_SIZE - 20.0, y + CELL_SIZE - 20.0, 8.0, BLUE);
                        draw_line(x + CELL_SIZE - 20.0, y + 20.0, x + 20.0, y + CELL_SIZE - 20.0, 8.0, BLUE);
                    }
                    CellState::Occupied(Player::O) => {
                        draw_circle_lines(x + CELL_SIZE / 2.0, y + CELL_SIZE / 2.0, CELL_SIZE / 3.0, 8.0, RED);
                    }
                    CellState::Empty => {}
                }
            }
        }

        let status = format!(
            "Player: {}",
            match game.current_player {
                Player::X => "X (Blue)",
                Player::O => "O (Red)",
            }
        );
        draw_text(&status, 20.0, 20.0, 30.0, DARKGRAY);

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            if mouse_y >= board_start_y && mouse_y <= board_start_y + board_width &&
               mouse_x >= board_start_x && mouse_x <= board_start_x + board_width {
                let col = ((mouse_x - board_start_x) / CELL_SIZE) as usize;
                let row = ((mouse_y - board_start_y) / CELL_SIZE) as usize;
                if row < BOARD_SIZE && col < BOARD_SIZE {
                    game.make_move(row, col);
                }
            }
        }

        next_frame().await;
    }
}

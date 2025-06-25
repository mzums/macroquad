use macroquad::prelude::*;

const BOARD_SIZE: usize = 3;
const CELL_SIZE: f32 = 150.0;
const BOARD_PADDING: f32 = 40.0;
const LINE_THICKNESS: f32 = 6.0;
const WIN_LINE_THICKNESS: f32 = 10.0;

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

enum GameState {
    Playing,
    Win(Player, WinType),
    Draw,
}

enum WinType {
    Row(usize),
    Column(usize),
    Diagonal,
    AntiDiagonal,
}

struct Game {
    board: [[CellState; BOARD_SIZE]; BOARD_SIZE],
    current_player: Player,
    game_state: GameState,
}

impl Game {
    fn new() -> Self {
        Self {
            board: [[CellState::Empty; BOARD_SIZE]; BOARD_SIZE],
            current_player: Player::X,
            game_state: GameState::Playing,
        }
    }

    fn make_move(&mut self, row: usize, col: usize) {
        if let GameState::Playing = self.game_state {
            if let CellState::Empty = self.board[row][col] {
                self.board[row][col] = CellState::Occupied(self.current_player);
                self.check_game_state(row, col);
                self.switch_player();
            }
        }
    }

    fn switch_player(&mut self) {
        self.current_player = match self.current_player {
            Player::X => Player::O,
            Player::O => Player::X,
        };
    }

    fn check_game_state(&mut self, row: usize, col: usize) {
        let player = match self.board[row][col] {
            CellState::Occupied(p) => p,
            _ => return,
        };

        if self.board[row].iter().all(|&cell| {
            matches!(cell, CellState::Occupied(p) if p == player)
        }) {
            self.game_state = GameState::Win(player, WinType::Row(row));
            return;
        }

        if (0..BOARD_SIZE).all(|r| {
            matches!(self.board[r][col], CellState::Occupied(p) if p == player)
        }) {
            self.game_state = GameState::Win(player, WinType::Column(col));
            return;
        }

        if row == col && (0..BOARD_SIZE).all(|i| {
            matches!(self.board[i][i], CellState::Occupied(p) if p == player)
        }) {
            self.game_state = GameState::Win(player, WinType::Diagonal);
            return;
        }

        if row + col == BOARD_SIZE - 1 && (0..BOARD_SIZE).all(|i| {
            matches!(self.board[i][BOARD_SIZE - 1 - i], CellState::Occupied(p) if p == player)
        }) {
            self.game_state = GameState::Win(player, WinType::AntiDiagonal);
            return;
        }

        if self.board.iter().flatten().all(|&cell| {
            !matches!(cell, CellState::Empty)
        }) {
            self.game_state = GameState::Draw;
        }
    }

    fn reset(&mut self) {
        *self = Game::new();
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Tic Tac Toe".to_owned(),
        window_width: 800,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

fn draw_centered_text(text: &str, font_size: u16, y: f32, color: Color) {
    let dim = measure_text(text, None, font_size, 1.0);
    draw_text(
        text,
        screen_width() / 2.0 - dim.width / 2.0,
        y,
        font_size as f32,
        color,
    );
}

fn draw_grid_lines(start_x: f32, start_y: f32, size: f32, line_thickness: f32) {
    for i in 0..=BOARD_SIZE {
        let pos = start_x + i as f32 * size;
        draw_line(
            pos,
            start_y,
            pos,
            start_y + size * BOARD_SIZE as f32,
            line_thickness,
            Color::from_rgba(180, 180, 180, 255),
        );
        draw_line(
            start_x,
            start_y + i as f32 * size,
            start_x + size * BOARD_SIZE as f32,
            start_y + i as f32 * size,
            line_thickness,
            Color::from_rgba(180, 180, 180, 255),
        );
    }
}

fn draw_win_line(
    start_x: f32,
    start_y: f32,
    width: f32,
    win_type: &WinType,
    color: Color,
) {
    match win_type {
        WinType::Row(row) => {
            let y = start_y + *row as f32 * CELL_SIZE + CELL_SIZE / 2.0;
            draw_line(start_x, y, start_x + width, y, WIN_LINE_THICKNESS, color);
        }
        WinType::Column(col) => {
            let x = start_x + *col as f32 * CELL_SIZE + CELL_SIZE / 2.0;
            draw_line(x, start_y, x, start_y + width, WIN_LINE_THICKNESS, color);
        }
        WinType::Diagonal => {
            draw_line(start_x, start_y, start_x + width, start_y + width, WIN_LINE_THICKNESS, color);
        }
        WinType::AntiDiagonal => {
            draw_line(
                start_x + width,
                start_y,
                start_x,
                start_y + width,
                WIN_LINE_THICKNESS,
                color,
            );
        }
    }
}

fn draw_x_o(board_start_x: f32, board_start_y: f32, game: &Game, hovered_cell: &mut Option<(usize, usize)>) {
    *hovered_cell = None;
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            let x = board_start_x + col as f32 * CELL_SIZE;
            let y = board_start_y + row as f32 * CELL_SIZE;
            
            let (mouse_x, mouse_y) = mouse_position();
            let hover = mouse_x >= x && mouse_x <= x + CELL_SIZE &&
                        mouse_y >= y && mouse_y <= y + CELL_SIZE;
            
            if hover && matches!(game.board[row][col], CellState::Empty) {
                *hovered_cell = Some((row, col));
                draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, Color::from_rgba(40, 40, 40, 255));
            }
            
            match game.board[row][col] {
                CellState::Occupied(Player::X) => {
                    let center_x = x + CELL_SIZE / 2.0;
                    let center_y = y + CELL_SIZE / 2.0;
                    let radius = CELL_SIZE / 3.0;
                    draw_line(
                        center_x - radius,
                        center_y - radius,
                        center_x + radius,
                        center_y + radius,
                        LINE_THICKNESS * 2.0,
                        Color::from_rgba(52, 152, 219, 255),
                    );
                    draw_line(
                        center_x + radius,
                        center_y - radius,
                        center_x - radius,
                        center_y + radius,
                        LINE_THICKNESS * 2.0,
                        Color::from_rgba(52, 152, 219, 255),
                    );
                }
                CellState::Occupied(Player::O) => {
                    let center_x = x + CELL_SIZE / 2.0;
                    let center_y = y + CELL_SIZE / 2.0;
                    let radius = CELL_SIZE / 3.0;
                    draw_circle_lines(
                        center_x,
                        center_y,
                        radius,
                        LINE_THICKNESS * 2.0,
                        Color::from_rgba(231, 76, 60, 255),
                    );
                }
                CellState::Empty => {
                    if hover {
                        let center_x = x + CELL_SIZE / 2.0;
                        let center_y = y + CELL_SIZE / 2.0;
                        let radius = CELL_SIZE / 4.0;
                        
                        match game.current_player {
                            Player::X => {
                                draw_line(
                                    center_x - radius,
                                    center_y - radius,
                                    center_x + radius,
                                    center_y + radius,
                                    LINE_THICKNESS,
                                    Color::from_rgba(52, 152, 219, 100),
                                );
                                draw_line(
                                    center_x + radius,
                                    center_y - radius,
                                    center_x - radius,
                                    center_y + radius,
                                    LINE_THICKNESS,
                                    Color::from_rgba(52, 152, 219, 100),
                                );
                            }
                            Player::O => {
                                draw_circle_lines(
                                    center_x,
                                    center_y,
                                    radius,
                                    LINE_THICKNESS,
                                    Color::from_rgba(231, 76, 60, 100),
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();
    let mut hovered_cell: Option<(usize, usize)> = None;
    let mut button_hovered = false;

    loop {
        clear_background(Color::from_rgba(0, 0, 0, 255));

        let board_width = CELL_SIZE * BOARD_SIZE as f32;
        let board_start_x = (screen_width() - board_width) / 2.0;
        let board_start_y = (screen_height() - board_width) / 2.0 - 30.0;

        draw_centered_text("TIC-TAC-TOE", 60, 70.0, Color::from_rgba(200, 200, 200, 255));

        draw_rectangle(
            board_start_x - 20.0,
            board_start_y - 20.0,
            board_width + 40.0,
            board_width + 40.0,
            Color::from_rgba(20, 20, 20, 255),
        );
        draw_rectangle_lines(
            board_start_x - 20.0,
            board_start_y - 20.0,
            board_width + 40.0,
            board_width + 40.0,
            2.0,
            Color::from_rgba(100, 100, 100, 255),
        );

        draw_grid_lines(board_start_x, board_start_y, CELL_SIZE, LINE_THICKNESS);

        if let GameState::Win(player, ref win_type) = game.game_state {
            draw_win_line(
                board_start_x,
                board_start_y,
                board_width,
                win_type,
                match player {
                    Player::X => Color::from_rgba(52, 152, 219, 255),
                    Player::O => Color::from_rgba(231, 76, 60, 255),
                },
            );
        }

        draw_x_o(board_start_x, board_start_y, &game, &mut hovered_cell);

        let status = match game.game_state {
            GameState::Playing => format!("Player {}'s turn", 
                match game.current_player {
                    Player::X => "X",
                    Player::O => "O",
                }),
            GameState::Win(player, _) => format!("Player {} wins!", 
                match player {
                    Player::X => "X",
                    Player::O => "O",
                }),
            GameState::Draw => "It's a draw!".to_string(),
        };

        let status_color = match game.game_state {
            GameState::Win(Player::X, _) => Color::from_rgba(52, 152, 219, 255),
            GameState::Win(Player::O, _) => Color::from_rgba(231, 76, 60, 255),
            _ => Color::from_rgba(100, 100, 100, 255),
        };
        
        draw_centered_text(&status, 40, board_start_y + board_width + 70.0, status_color);

        let button_text = "PLAY AGAIN";
        let button_width = 220.0;
        let button_height = 60.0;
        let button_x = screen_width() / 2.0 - button_width / 2.0;
        let button_y = board_start_y + board_width + 120.0;
        
        let (mouse_x, mouse_y) = mouse_position();
        button_hovered = mouse_x >= button_x && mouse_x <= button_x + button_width &&
                         mouse_y >= button_y && mouse_y <= button_y + button_height;
        
        let button_color = if button_hovered {
            Color::from_rgba(46, 204, 113, 255)
        } else {
            Color::from_rgba(39, 174, 96, 255)
        };
        
        draw_rectangle(
            button_x,
            button_y,
            button_width,
            button_height,
            button_color,
        );
        
        draw_rectangle_lines(
            button_x,
            button_y,
            button_width,
            button_height,
            3.0,
            Color::from_rgba(30, 130, 76, 255),
        );
        
        draw_centered_text(
            button_text,
            36,
            button_y + button_height / 2.0 + 12.0,
            WHITE,
        );

        if is_mouse_button_pressed(MouseButton::Left) {
            if let Some((row, col)) = hovered_cell {
                game.make_move(row, col);
            }
            
            if button_hovered {
                game.reset();
            }
        }

        next_frame().await;
    }
}
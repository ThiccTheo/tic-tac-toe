use {
    super::{app::App, state::Action},
    ggez::{
        event::EventHandler,
        graphics::{Canvas, Color, DrawParam, Image, Text, TextLayout},
        input::{keyboard::KeyCode, mouse::MouseButton},
        mint::Point2,
        Context,
    },
    std::io::{stdout, Write},
};

pub struct Game {
    board: [[char; Self::WIDTH]; Self::HEIGHT],
    partition: Image,
    turn: char,
    is_game_over: bool,
    is_active: bool,
}

impl Game {
    const WIDTH: usize = 3;
    const HEIGHT: usize = 3;
    const CELL_WIDTH: f32 = App::WIDTH / 3.0;
    const CELL_HEIGHT: f32 = App::HEIGHT / 3.0;
    const TEXT_SCALE: f32 = 300.0;
    const PLAYER1: char = 'X';
    const PLAYER2: char = 'O';
    const PLAYER1_COLOR: Color = Color::new(255.0 / 255.0, 0.0 / 255.0, 0.0 / 255.0, 255.0 / 255.0);
    const PLAYER2_COLOR: Color = Color::new(0.0 / 255.0, 0.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0);
    const EMPTY: char = ' ';

    pub fn new(ctx: &mut Context) -> Self {
        print!("\x1B[2J\x1B[1;1H");
        stdout().flush().unwrap();

        Self {
            board: [
                [Self::EMPTY, Self::EMPTY, Self::EMPTY],
                [Self::EMPTY, Self::EMPTY, Self::EMPTY],
                [Self::EMPTY, Self::EMPTY, Self::EMPTY],
            ],
            partition: Image::from_path(&ctx.gfx, "\\board.png").unwrap(),
            turn: Self::PLAYER1,
            is_game_over: false,
            is_active: true,
        }
    }
}

impl EventHandler<Action> for Game {
    fn update(&mut self, ctx: &mut Context) -> Result<(), Action> {
        if !self.is_game_over {
            if ctx.mouse.button_just_pressed(MouseButton::Left) {
                let x = (ctx.mouse.position().x / Self::CELL_WIDTH) as usize;
                let y = (ctx.mouse.position().y / Self::CELL_HEIGHT) as usize;

                if self.board[y][x] == Self::EMPTY {
                    self.board[y][x] = self.turn;

                    self.turn = if self.turn == Self::PLAYER1 {
                        Self::PLAYER2
                    } else {
                        Self::PLAYER1
                    }
                }
            }

            for y in 0..Self::HEIGHT {
                if self.board[y][0] != Self::EMPTY
                    && self.board[y][0] == self.board[y][1]
                    && self.board[y][0] == self.board[y][2]
                {
                    self.is_game_over = true;
                    println!("{} wins!", self.board[y][0]);
                }
            }

            for x in 0..Self::WIDTH {
                if self.board[0][x] != Self::EMPTY
                    && self.board[0][x] == self.board[1][x]
                    && self.board[0][x] == self.board[2][x]
                {
                    self.is_game_over = true;
                    println!("{} wins!", self.board[0][x]);
                }
            }

            if self.board[0][0] != Self::EMPTY
                && self.board[0][0] == self.board[1][1]
                && self.board[0][0] == self.board[2][2]
            {
                self.is_game_over = true;
                println!("{} wins!", self.board[0][0]);
            }

            if self.board[2][0] != Self::EMPTY
                && self.board[2][0] == self.board[1][1]
                && self.board[2][0] == self.board[0][2]
            {
                self.is_game_over = true;
                println!("{} wins!", self.board[2][0]);
            }

            let mut flag = false;

            'outer: for y in 0..Self::HEIGHT {
                for x in 0..Self::WIDTH {
                    if self.board[y][x] == Self::EMPTY {
                        flag = true;
                        break 'outer;
                    }
                }
            }

            if !self.is_game_over && !flag {
                self.is_game_over = true;
                println!("Cat's game!");
            }

        } else {
            if ctx.keyboard.is_key_just_pressed(KeyCode::Space) {
                self.is_active = false;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), Action> {
        let mut canvas = Canvas::from_frame(&ctx.gfx, Color::WHITE);

        canvas.draw(&self.partition, DrawParam::default());

        for y in 0..Self::HEIGHT {
            for x in 0..Self::WIDTH {
                let mut text = Text::new(self.board[y][x]);
                text.set_scale(Self::TEXT_SCALE);
                text.set_layout(TextLayout::center());
                canvas.draw(
                    &text,
                    DrawParam::default()
                        .dest(Point2 {
                            x: x as f32 * Self::CELL_WIDTH + Self::CELL_WIDTH / 2.0,
                            y: y as f32 * Self::CELL_HEIGHT + Self::CELL_HEIGHT / 2.0,
                        })
                        .color(if self.board[y][x] == Self::PLAYER1 {
                            Self::PLAYER1_COLOR
                        } else {
                            Self::PLAYER2_COLOR
                        }),
                );
            }
        }

        canvas.finish(&mut ctx.gfx).unwrap();

        if !self.is_active {
            Err(Action::Change(Box::new(Self::new(ctx))))
        } else {
            Ok(())
        }
    }
}

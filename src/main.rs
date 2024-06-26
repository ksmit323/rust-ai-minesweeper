use ggez::event::{self, EventHandler, MouseButton};
use ggez::graphics::{self, Color, DrawMode, Image, Mesh, PxScale, Rect, Text, TextFragment};
use ggez::*;
use rust_ai_minesweeper::game_logic::*;
use std::collections::HashSet;

const HEIGHT: usize = 8;
const WIDTH: usize = 8;
const NUM_MINES: usize = 8;
const TILE_SIZE: f32 = 50.0;

struct State {
    game: Minesweeper,
    ai: MinesweeperAI,
    revealed: HashSet<(usize, usize)>,
    flags: HashSet<(usize, usize)>,
    lost: bool,
    instructions: bool,
    flag_image: Image,
    mine_image: Image,
}

impl State {
    pub fn new(ctx: &mut Context, height: usize, width: usize, num_of_mines: usize) -> Self {
        Self {
            game: Minesweeper::new(height, width, num_of_mines),
            ai: MinesweeperAI::new(height, width),
            revealed: HashSet::new(),
            flags: HashSet::new(),
            lost: false,
            instructions: false,
            flag_image: Image::from_path(ctx, "/flag.png").unwrap(),
            mine_image: Image::from_path(ctx, "/mine.png").unwrap(),
        }
    }
}

impl EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        // TODO: add instructions before drawing board

        // Draw the board
        let margin = 3.0; // margin between each square
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                let x = j as f32 * TILE_SIZE;
                let y = i as f32 * TILE_SIZE;

                // Draw the outer rectangle (border)
                let outer_rect = Mesh::new_rectangle(
                    ctx,
                    DrawMode::stroke(1.0),
                    Rect::new(x, y, TILE_SIZE, TILE_SIZE),
                    Color::WHITE,
                )?;
                canvas.draw(&outer_rect, graphics::DrawParam::default());

                // Draw the inner rectangle
                let inner_rect = Mesh::new_rectangle(
                    ctx,
                    DrawMode::fill(),
                    Rect::new(
                        x + margin,
                        y + margin,
                        TILE_SIZE - margin * 2.0,
                        TILE_SIZE - margin * 2.0,
                    ),
                    Color::from_rgb(125, 125, 125),
                )?;
                canvas.draw(&inner_rect, graphics::DrawParam::default());

                // Draw number
                if self.revealed.contains(&(i, j)) {
                    let text = Text::new(TextFragment {
                        text: self.game.nearby_mines((i, j)).to_string(),
                        color: Some(Color::BLACK),
                        font: Some("LiberationMono-Regular".into()),
                        scale: Some(PxScale::from(30.0)),
                    });
                    canvas.draw(
                        &text,
                        graphics::DrawParam::default().dest([x + 15.0, y + 10.0]),
                    );

                // Draw flags
                } else if self.flags.contains(&(i, j)) {
                    // let text = Text::new(TextFragment {
                    //     text: "F".to_string(),
                    //     color: Some(Color::BLACK),
                    //     font: Some("LiberationMono-Regular".into()),
                    //     scale: Some(PxScale::from(30.0)),
                    // });
                    let scale = [
                        TILE_SIZE / self.flag_image.width() as f32,
                        TILE_SIZE / self.flag_image.height() as f32,
                    ];
                    canvas.draw(
                        &self.flag_image,
                        graphics::DrawParam::default()
                            .dest([x, y])
                            .scale(scale),
                    );

                // Draw mines
                } else if self.game.is_mine((i, j)) && self.lost {
                    // let text = Text::new(TextFragment {
                    //     text: "M".to_string(),
                    //     color: Some(Color::BLACK),
                    //     font: Some("LiberationMono-Regular".into()),
                    //     scale: Some(PxScale::from(30.0)),
                    // });
                    let scale = [
                        TILE_SIZE / self.mine_image.width() as f32,
                        TILE_SIZE / self.mine_image.height() as f32,
                    ];
                    canvas.draw(
                        &self.mine_image,
                        graphics::DrawParam::default()
                            .dest([x, y])
                            .scale(scale),
                    );
                }
            }
        }

        // Draw AI move button
        let rect_length = 150.0;
        let rect_width = 50.0;
        let x_ai_button = 450.0;
        let y_ai_button = 50.0;
        let x_text = x_ai_button + 20.0;
        let y_text = y_ai_button + 10.0;
        let ai_button = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(x_ai_button, y_ai_button, rect_length, rect_width),
            Color::WHITE,
        )?;
        canvas.draw(&ai_button, graphics::DrawParam::default());

        let ai_text = Text::new(TextFragment {
            text: "AI Move".to_string(),
            color: Some(Color::BLACK),
            font: Some("LiberationMono-Regular".into()),
            scale: Some(PxScale::from(30.0)),
        });
        canvas.draw(
            &ai_text,
            graphics::DrawParam::default().dest([x_text, y_text]),
        );

        // Draw the reset button
        let x_reset_button = x_ai_button;
        let y_reset_button = y_ai_button + 75.0;
        let x_text = x_reset_button + 30.0;
        let y_text = y_reset_button + 10.0;
        let reset_button = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(x_reset_button, y_reset_button, rect_length, rect_width),
            Color::WHITE,
        )?;
        canvas.draw(&reset_button, graphics::DrawParam::default());

        let reset_text = Text::new(TextFragment {
            text: "Reset".to_string(),
            color: Some(Color::BLACK),
            font: Some("LiberationMono-Regular".into()),
            scale: Some(PxScale::from(30.0)),
        });
        canvas.draw(
            &reset_text,
            graphics::DrawParam::default().dest([x_text, y_text]),
        );

        // Draw winner or loser text
        if self.lost {
            let mut text = graphics::Text::new("Loser!");
            text.set_scale(200.0);
            let dest_point = [125.0, 400.0];
            canvas.draw(&text, graphics::DrawParam::default().dest(dest_point));
        } else {
            if self.game.mines == self.flags {
                let mut text = graphics::Text::new("Winner!");
                text.set_scale(200.0);
                let dest_point = [75.0, 400.0];
                canvas.draw(&text, graphics::DrawParam::default().dest(dest_point));
            }
        }

        canvas.finish(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        if button == MouseButton::Left {
            let mut mv: Option<(usize, usize)> = None;

            let px_height = HEIGHT as f32 * TILE_SIZE;
            let px_width = WIDTH as f32 * TILE_SIZE;

            // human player made the move
            if x >= 0.0 && x <= px_height && y >= 0.0 && y <= px_width {
                let col = (x / TILE_SIZE) as usize;
                let row = (y / TILE_SIZE) as usize;
                if !self.flags.contains(&(row, col)) && !self.revealed.contains(&(row, col)) {
                    mv = Some((row, col));
                }
            }

            // AI Move button clicked
            if x >= 450.0 && x <= 600.0 && y >= 50.0 && y <= 100.0 && !self.lost {
                if let Some(ai_move) = self
                    .ai
                    .make_safe_move()
                    .or_else(|| self.ai.make_random_move())
                {
                    mv = Some(ai_move);
                } else {
                    self.flags = self.ai.known_mines.clone();
                }
            }

            // Reset button clicked
            if x >= 450.0 && x <= 600.0 && y >= 125.0 && y <= 175.0 {
                self.revealed = HashSet::new();
                self.flags = HashSet::new();
                self.lost = false;
                self.game = Minesweeper::new(HEIGHT, WIDTH, NUM_MINES);
                self.ai = MinesweeperAI::new(HEIGHT, WIDTH);
                self.instructions = true;
                return Ok(());
            }

            // Make move and update knowledge
            if let Some(mv) = mv {
                if self.game.is_mine(mv) {
                    self.lost = true;
                } else {
                    self.revealed.insert(mv);
                    self.ai.add_knowledge(mv, self.game.nearby_mines(mv))
                }
            }
        }
        if button == MouseButton::Right {
            let col = (x / TILE_SIZE) as usize;
            let row = (y / TILE_SIZE) as usize;
            if row < HEIGHT && col < WIDTH {
                if self.game.is_mine((row, col)) {
                    self.flags.insert((row, col));
                } else {
                    self.lost = true;
                }
            }
        }
        Ok(())
    }
}

fn main() {
    // Make context and an event loop
    let c = conf::Conf::new();
    let (mut ctx, event_loop) = ContextBuilder::new("Minesweeper", "Ken")
        .default_conf(c)
        .add_resource_path("./resources")
        .build()
        .unwrap();

    let state = State::new(&mut ctx, HEIGHT, WIDTH, NUM_MINES);

    // Launch the game by starting the event loop
    event::run(ctx, event_loop, state);
}

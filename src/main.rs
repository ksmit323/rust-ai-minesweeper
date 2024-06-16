use ggez::event::{self, EventHandler, MouseButton};
use ggez::graphics::{self, Color, DrawMode, Mesh, PxScale, Rect, Text, TextFragment};
use ggez::*;
use rust_ai_minesweeper::game_logic::*;

const HEIGHT: usize = 8;
const WIDTH: usize = 8;
const NUM_MINES: usize = 8;

struct State {
    game: Minesweeper,
    ai: MinesweeperAI,
    board: [[bool; HEIGHT]; WIDTH],
}

impl State {
    pub fn new(height: usize, width: usize, num_of_mines: usize) -> Self {
        Self {
            game: Minesweeper::new(height, width, num_of_mines),
            ai: MinesweeperAI::new(height, width),
            board: [[false; HEIGHT]; WIDTH],
        }
    }
}

impl EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        // Draw the board
        let tile_size = 50.0;
        let margin = 3.0; // margin between each square
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                let x = j as f32 * tile_size;
                let y = i as f32 * tile_size;

                // Draw the outer rectangle (border)
                let outer_rect = Mesh::new_rectangle(
                    ctx,
                    DrawMode::stroke(1.0),
                    Rect::new(x, y, tile_size, tile_size),
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
                        tile_size - margin * 2.0,
                        tile_size - margin * 2.0,
                    ),
                    Color::from_rgb(125, 125, 125),
                )?;
                canvas.draw(&inner_rect, graphics::DrawParam::default());

                // Draw the number of mines in each square
                let num_of_mines = self.game.nearby_mines((i, j)).to_string();
                if self.board[i][j] {
                    let text = Text::new(TextFragment {
                        text: num_of_mines,
                        color: Some(Color::BLACK),
                        font: Some("LiberationMono-Regular".into()),
                        scale: Some(PxScale::from(30.0)),
                    });
                    canvas.draw(
                        &text,
                        graphics::DrawParam::default().dest([x + 15.0, y + 10.0]),
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
            let col = (x / 30.0) as usize;
            let row = (y / 30.0) as usize;
            if row < self.game.height && col < self.game.width {
                self.board[row][col] = true;
            }
        }

        Ok(())
    }
}

fn main() {
    // Make context and an event loop
    let c = conf::Conf::new();
    let (ctx, event_loop) = ContextBuilder::new("Minesweeper", "Ken")
        .default_conf(c)
        .build()
        .unwrap();

    let state = State::new(HEIGHT, WIDTH, NUM_MINES);

    // Launch the game by starting the event loop
    event::run(ctx, event_loop, state);
}

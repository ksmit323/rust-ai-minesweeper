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

impl ggez::event::EventHandler<GameError> for State {
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
                    let mut text = Text::new(TextFragment {
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
        canvas.finish(ctx)?;
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

    let state = State {
        game: Minesweeper::new(HEIGHT, WIDTH, NUM_MINES),
        ai: MinesweeperAI::new(HEIGHT, WIDTH),
        board: [[false; HEIGHT]; WIDTH],
    };

    // Launch the game by starting the event loop
    event::run(ctx, event_loop, state);
}

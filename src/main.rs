use ggez::{Context, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, DrawParam, Image, Color, Drawable};
use ggez::mint::Point2;

struct MainState {
    emoji: Image,
    positions: Vec<Point2<f32>>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        // Load image using filesystem
        let emoji = Image::from_path(ctx, "/emoji.png")?;
        Ok(MainState {
            emoji,
            positions: Vec::new(),
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Create a canvas to draw on
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        for position in &self.positions {
            // Draw the emoji on the canvas
            self.emoji.draw(&mut canvas, DrawParam::default().dest(*position));
        }

        // Finish drawing and present the canvas
        canvas.finish(ctx)?;
        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) -> GameResult<()> {
        self.positions.push(Point2 { x, y });
        Ok(())
    }
}

fn main() -> GameResult<()> {
    let cb = ggez::ContextBuilder::new("mouse_emoji_trail", "Your Name");
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}

use tetra::graphics::text::{Font, Text};
use tetra::graphics::{self, Color, Rectangle, Texture};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;
const PADDLE_SPEED: f32 = 8.0;

fn main() -> tetra::Result {
    ContextBuilder::new("Pong", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}

struct Ball {
    texture: Texture,
    position: Vec2<f32>,
    vector: Vec2<f32>,
    speed: f32,
}

impl Ball {
    fn new(texture: Texture, position: Vec2<f32>, vector: Vec2<f32>, speed: f32) -> Ball {
        Ball {
            texture,
            position,
            vector,
            speed
        }
    }
    
    fn width(&self) -> f32 {
        self.texture.width() as f32
    }
    
    fn height(&self) -> f32 {
        self.texture.height() as f32
    }

    fn bounds(&self) -> Rectangle {
        Rectangle::new(
            self.position.x,
            self.position.y,
            self.width(),
            self.height(),
        )
    }

    fn reinitialize_ball(&mut self) {
        self.position = Vec2 {
            x:WINDOW_WIDTH / 2.0, 
            y:WINDOW_HEIGHT / 2.0
        };
        self.vector = Vec2 {
            x:1.5, 
            y:2.5
        };
        self.speed = 1.0;
    }
}

struct Paddle {
    texture: Texture,
    position: Vec2<f32>,
    great_shot: f32,
    score: u8,
    score_display: Text
}

impl Paddle {
    fn new(texture: Texture, position: Vec2<f32>, great_shot: f32, score: u8, score_display: Text) -> Paddle{
        Paddle {
            texture,
            position,
            great_shot,
            score,
            score_display
        }
    }

    fn width(&self) -> f32 {
        self.texture.width() as f32
    }
    
    fn height(&self) -> f32 {
        self.texture.height() as f32
    }

    fn bounds(&self) -> Rectangle {
        Rectangle::new(
            self.position.x,
            self.position.y,
            self.width(),
            self.height(),
        )
    }
}

struct GameState {
    player1: Paddle,
    player2: Paddle,
    ball: Ball,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        // player 1
        let player1_texture = Texture::new(ctx, "./resources/player1.png")?;
        let player1_position = Vec2::new(
            16.0,
            (WINDOW_HEIGHT - player1_texture.height() as f32) / 2.0,
        );
        let player1_great_shot = 1.0;
        let player1_score = 0;
        let score_1_display = Text::new(format!("{}", &player1_score), Font::vector(ctx, "./resources/leadcoat.ttf", 16.0)?,);

        // player 2
        let player2_texture = Texture::new(ctx, "./resources/player2.png")?;
        let player2_position = Vec2::new(
            WINDOW_WIDTH - player2_texture.width() as f32 - 16.0,
            (WINDOW_HEIGHT - player2_texture.height() as f32) / 2.0,
        );
        let player2_great_shot = 1.0;
        let player2_score = 0;
        let score_2_display = Text::new(format!("{}", &player2_score), Font::vector(ctx, "./resources/leadcoat.ttf", 16.0)?,);

        // ball
        let ball_texture = Texture::new(ctx, "./resources/ball.png")?;
        let ball_position = Vec2::new(
            WINDOW_WIDTH / 2.0,
            WINDOW_HEIGHT / 2.0,
        );
        let ball_vector = Vec2::new(
            1.5,
            2.5,
        );
        let ball_speed = 1.0;

        Ok(GameState {
            player1: Paddle::new(player1_texture, player1_position, player1_great_shot, player1_score, score_1_display),
            player2: Paddle::new(player2_texture, player2_position, player2_great_shot, player2_score, score_2_display),
            ball: Ball::new(ball_texture, ball_position, ball_vector, ball_speed),
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        // collides
        let player1_bounds = self.player1.bounds();
        let player2_bounds = self.player2.bounds();
        let ball_bounds = self.ball.bounds();
        
        if ball_bounds.intersects(&player1_bounds) {
            self.ball.vector.x *= -1.0;
            self.ball.speed += 0.1;
            self.player2.great_shot = 1.0;
        }
        if ball_bounds.intersects(&player2_bounds) {
            self.ball.vector.x *= -1.0;
            self.ball.speed += 0.1;
            self.player1.great_shot = 1.0;
        }

        // move ball
        self.ball.position.x += self.ball.vector.x * self.ball.speed * self.player1.great_shot * self.player2.great_shot;
        self.ball.position.y += self.ball.vector.y * self.ball.speed * self.player1.great_shot * self.player2.great_shot;
        if self.ball.position.y <= 0.0 || self.ball.position.y >= WINDOW_HEIGHT - self.ball.texture.height() as f32{
            self.ball.vector.y *= -1.0;
        }
        if self.ball.position.x >= WINDOW_WIDTH + 40.0 {
            self.player1.score += 1;
            self.player1.score_display = Text::new(format!("{}", self.player1.score), Font::vector(ctx, "./resources/leadcoat.ttf", 16.0)?,);
            self.player1.great_shot = 1.0;
            self.player2.great_shot = 1.0;
            self.ball.reinitialize_ball()
        }
        if self.ball.position.x <= -40.0 { 
            self.player2.score += 1;
            self.player2.score_display = Text::new(format!("{}", self.player2.score), Font::vector(ctx, "./resources/leadcoat.ttf", 16.0)?,);
            self.player1.great_shot = 1.0;
            self.player2.great_shot = 1.0;
            self.ball.reinitialize_ball()
        }

        // keyboard
        if input::is_key_down(ctx, Key::W) && self.player1.position.y > 0.0{
            self.player1.position.y -= PADDLE_SPEED;
        }

        if input::is_key_down(ctx, Key::S) && self.player1.position.y < WINDOW_HEIGHT - self.player1.texture.height() as f32{
            self.player1.position.y += PADDLE_SPEED;
        }

        if input::is_key_down(ctx, Key::Up) && self.player2.position.y > 0.0{
            self.player2.position.y -= PADDLE_SPEED;
        }

        if input::is_key_down(ctx, Key::Down) && self.player2.position.y < WINDOW_HEIGHT - self.player1.texture.height() as f32{
            self.player2.position.y += PADDLE_SPEED;
        }

        if input::is_key_pressed(ctx, Key::Left) && self.ball.position.x > self.player2.position.x - 40.0 {
            println!("GREAT SHOT!");
            self.player2.great_shot = 3.0;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        graphics::draw(ctx, &self.player1.texture, self.player1.position);
        graphics::draw(ctx, &self.player2.texture, self.player2.position);
        graphics::draw(ctx, &self.ball.texture, self.ball.position);
        graphics::draw(ctx, &self.player1.score_display, Vec2::new(WINDOW_WIDTH * 0.4, 16.0));
        graphics::draw(ctx, &self.player2.score_display, Vec2::new(WINDOW_WIDTH * 0.6, 16.0));


        Ok(())
    }
}


//! https://gamegix.com/simple/game
//! https://medium.freecodecamp.org/an-introduction-to-reinforcement-learning-4339519de419
use ggez::event::{self, KeyCode};
use ggez::{self, graphics, input::keyboard, nalgebra, timer, Context, ContextBuilder, GameResult};
use rand::Rng;

mod util;

const PLAYER_SIZE: f32 = 20.0;
const SCORE_SIZE: f32 = 30.0;
const BUG_SIZE: f32 = 10.0;

struct Bug {
    pos: nalgebra::Point2<f32>,
    vel: nalgebra::Vector2<f32>,
}

impl Bug {
    fn new(player_pos: nalgebra::Point2<f32>, screen_size: (f32, f32)) -> Bug {
        let mut rng = rand::thread_rng();
        let speed = rng.gen::<f32>() * 3.0 + 2.0;
        let velocity = if rng.gen::<bool>() {
            (speed, 0.0)
        } else {
            (0.0, speed)
        };
        Bug {
            pos: random_position(player_pos, screen_size, BUG_SIZE),
            vel: nalgebra::Vector2::new(velocity.0, velocity.1),
        }
    }
}

struct State {
    bugs: Vec<Bug>,
    score_pos: nalgebra::Point2<f32>,
    score_hit: u32,
    player_pos: nalgebra::Point2<f32>,
    screen_size: (f32, f32),
}

/// Generate random position without hitting player.
fn random_position(
    player_pos: nalgebra::Point2<f32>,
    screen_size: (f32, f32),
    size: f32,
) -> nalgebra::Point2<f32> {
    let mut rng = rand::thread_rng();
    let mut x_rand = rng.gen::<f32>() * (screen_size.0 - PLAYER_SIZE - size * 2.0) + (size / 2.0);
    let mut y_rand = rng.gen::<f32>() * (screen_size.1 - PLAYER_SIZE - size * 2.0) + (size / 2.0);
    // poor man's position prevention on the same row/column as player
    let half = size / 2.0;
    let player_half = PLAYER_SIZE / 2.0;
    if x_rand + half >= player_pos.x - player_half {
        x_rand += PLAYER_SIZE + size;
    }
    if y_rand + half >= player_pos.y - player_half {
        y_rand += PLAYER_SIZE + size;
    }
    nalgebra::Point2::new(x_rand, y_rand)
}

/// Test for collition between two entity.
fn collide(a: nalgebra::Point2<f32>, a_half: f32, b: nalgebra::Point2<f32>, b_half: f32) -> bool {
    a.x - a_half < b.x + b_half
        && a.x + a_half > b.x - b_half
        && a.y - a_half < b.y + b_half
        && a.y + a_half > b.y - b_half
}

impl event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while timer::check_update_time(ctx, 30) {
            // handle key events
            let player_speed = 5.0;
            let player_half = PLAYER_SIZE / 2.0;

            if cfg!(feature = "bot") {
            } else {
                if keyboard::is_key_pressed(ctx, KeyCode::Left)
                    && self.player_pos.x - player_half > 0.0
                {
                    self.player_pos.x -= player_speed;
                } else if keyboard::is_key_pressed(ctx, KeyCode::Right)
                    && self.player_pos.x + player_half < self.screen_size.0
                {
                    self.player_pos.x += player_speed;
                }
                if keyboard::is_key_pressed(ctx, KeyCode::Up)
                    && self.player_pos.y - player_half > 0.0
                {
                    self.player_pos.y -= player_speed;
                } else if keyboard::is_key_pressed(ctx, KeyCode::Down)
                    && self.player_pos.y + player_half < self.screen_size.1
                {
                    self.player_pos.y += player_speed;
                }
            }

            // score system
            let score_half = SCORE_SIZE / 2.0;
            if collide(self.score_pos, score_half, self.player_pos, player_half) {
                self.score_pos = random_position(self.player_pos, self.screen_size, SCORE_SIZE);
                self.score_hit += 1;
                self.bugs.push(Bug::new(self.player_pos, self.screen_size));
                log::info!("Score: {}", self.score_hit);
            }

            // buggy system
            for bug in &mut self.bugs {
                bug.pos += bug.vel;

                // reflect bug off screen
                let half = BUG_SIZE / 2.0;
                if bug.pos.x - half < 0.0 || bug.pos.x + half > self.screen_size.0 {
                    bug.vel.x *= -1.0;
                } else if bug.pos.y - half < 0.0 || bug.pos.y + half > self.screen_size.1 {
                    bug.vel.y *= -1.0;
                }

                // handle collisions
                if collide(bug.pos, half, self.player_pos, player_half) {
                    log::info!("Game over! High score {}", self.score_hit);
                    ggez::quit(ctx);
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        let half = SCORE_SIZE / 2.0;
        let score = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(-half, -half, SCORE_SIZE, SCORE_SIZE),
            graphics::Color::new(0.0, 0.0, 1.0, 1.0),
        )?;
        graphics::draw(ctx, &score, (self.score_pos,))?;

        let half = PLAYER_SIZE / 2.0;
        let player = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(-half, -half, PLAYER_SIZE, PLAYER_SIZE),
            graphics::Color::new(0.0, 1.0, 0.0, 1.0),
        )?;
        graphics::draw(ctx, &player, (self.player_pos,))?;

        let half = BUG_SIZE / 2.0;
        let bug_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(-half, -half, BUG_SIZE, BUG_SIZE),
            graphics::Color::new(1.0, 0.0, 0.0, 1.0),
        )?;
        for bug in &self.bugs {
            graphics::draw(ctx, &bug_mesh, (bug.pos,))?;
        }

        graphics::present(ctx)?;
        let fps = 1000.0 / timer::average_delta(ctx).subsec_millis() as f32;
        log::debug!("Framerate: {}", fps);
        Ok(())
    }
}

fn main() {
    util::setup_logger();

    let (mut ctx, mut event_loop) = ContextBuilder::new("bugs", "awesome").build().unwrap();

    let screen_size = (ctx.conf.window_mode.width, ctx.conf.window_mode.height);
    let half = PLAYER_SIZE / 2.0;
    let player_pos = nalgebra::Point2::new(screen_size.0 / 2.0 - half, screen_size.1 / 2.0 - half);
    let state = &mut State {
        bugs: vec![Bug::new(player_pos, screen_size)],
        score_pos: random_position(player_pos, screen_size, SCORE_SIZE),
        score_hit: 0,
        player_pos,
        screen_size,
    };

    event::run(&mut ctx, &mut event_loop, state).unwrap();
}

use piston_window::{ellipse, Context, Graphics};

use crate::game::Position;

const BALL_SPEED: f64 = 2.5;
pub struct Direction {
    pub x: f64,
    pub y: f64,
}
pub struct Ball {
    pub position: Position,
    pub r: f64,
    pub direction: Direction,
    pub ball_speed: f64,
}

impl Ball {
    pub fn new(window_w: f64, window_h: f64) -> Ball {
        Ball {
            position: Position {
                x: window_w / 2.0,
                y: window_h / 2.0,
            },
            r: 10.0,
            direction: Direction {
                x: BALL_SPEED,
                y: BALL_SPEED,
            },
            ball_speed: 1.0,
        }
    }

    pub fn reset(&mut self, window_w: f64, window_h: f64) {
        self.position.x = window_w / 2.0;
        self.position.y = window_h / 2.0;
        self.ball_speed = 1.0;
        self.direction.x = BALL_SPEED;
        self.direction.y = BALL_SPEED;
    }

    pub fn increase_speed(&mut self) {
        if self.ball_speed < 3.0 {
            self.ball_speed += 0.05;
        }
    }

    pub fn update(&mut self) {
        self.position.x += self.direction.x * self.ball_speed;
        self.position.y += self.direction.y * self.ball_speed;
    }

    pub fn change_x_direction(&mut self) {
        self.direction.x *= -1.0;
    }

    pub fn change_y_direction(&mut self) {
        self.direction.y *= -1.0;
    }

    pub fn draw<G: Graphics>(&self, context: Context, g: &mut G) {
        ellipse(
            [1.0; 4],
            [self.position.x, self.position.y, self.r, self.r],
            context.transform,
            g,
        );
    }
}

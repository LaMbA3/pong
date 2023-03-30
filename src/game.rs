use piston_window::*;
use crate::{ball::Ball, pong::Pong};
use std::collections::HashMap;


pub struct Position{
    pub x: f64,
    pub y: f64
}


pub struct Game{
    window_width: f64,
    window_height: f64,
    left_pong: Pong,
    right_pong: Pong,
    ball: Ball,
    player1_score: u32,
    player2_score: u32,
    pressed_keys: HashMap<Key, bool>
}

const PONG_SPEED: f64 = 5.0;

impl Game {
    pub fn new(window_width: f64, window_height: f64)-> Game{
        let left_pong_position = Position{x: 0.0, y: window_height/2.0};
        let right_pong_position = Position{x: window_width, y: window_height/2.0};
        
        Game {
            window_height,
            window_width,
            left_pong: Pong::new(left_pong_position, 10.0, window_height*0.15),
            right_pong: Pong::new(right_pong_position, 10.0, window_height*0.15),
            ball: Ball::new(window_width, window_height),
            player1_score: 0,
            player2_score: 0,
            pressed_keys: HashMap::new()
        }
    }

    fn move_pong(&mut self){
        if self.pressed_keys.contains_key(&Key::W){
            if self.left_pong.position.y - PONG_SPEED >= 0.0 {
                self.left_pong.move_by(-PONG_SPEED);
            }
        }
        if self.pressed_keys.contains_key(&Key::S){
            if self.left_pong.position.y + self.left_pong.height + PONG_SPEED <= self.window_height {
                self.left_pong.move_by(PONG_SPEED);
            }
        }
        if self.pressed_keys.contains_key(&Key::Up){
            if self.right_pong.position.y - PONG_SPEED >= 0.0 {
                self.right_pong.move_by(-PONG_SPEED);
            }
        }
        if self.pressed_keys.contains_key(&Key::Down){
            if self.right_pong.position.y + self.right_pong.height + PONG_SPEED <= self.window_height {
                self.right_pong.move_by(PONG_SPEED);
            }
        }
    }
    pub fn handle_key_press(&mut self,key: Key){
        self.pressed_keys.insert(key, true);
    }

    pub fn handle_key_release(&mut self, key: Key){
        self.pressed_keys.remove(&key);
    }

    pub fn draw<G: Graphics>(&mut self, context: Context, graphics: &mut G){
        self.ball.update();
        self.check_win();
        self.left_pong.draw(context,graphics);
        self.right_pong.draw(context, graphics);
        self.check_collision();
        self.ball.draw(context,graphics);
        self.move_pong();
    }

    fn check_collision(&mut self){
        if self.ball.position.x <= self.left_pong.position.x + self.left_pong.width{
            if self.ball.position.y >= self.left_pong.position.y && self.ball.position.y <= self.left_pong.position.y + self.left_pong.height{
                self.ball.change_x_direction();
            }
        }
        if self.ball.position.x >= self.right_pong.position.x - self.right_pong.width{
            if self.ball.position.y >= self.right_pong.position.y && self.ball.position.y <= self.right_pong.position.y + self.right_pong.height{
                self.ball.change_x_direction();
            }
        }


        if self.ball.position.y <= 0.0 || self.ball.position.y >= self.window_height{
            self.ball.change_y_direction();
        }
    }

    fn check_win(&mut self){
        if self.ball.position.x <= 0.0{
            self.player2_score += 1;
            self.ball.reset(self.window_width, self.window_height);
        }
        if self.ball.position.x >= self.window_width{
            self.player1_score += 1;
            self.ball.reset(self.window_width, self.window_height);
        }
    }

}

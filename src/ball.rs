use piston_window::{Graphics, Context, ellipse};

use crate::game::Position;


pub struct Direction{
    pub x: f64,
    pub y: f64
}
pub struct Ball{
    pub position: Position,
    pub r: f64,
    pub direction: Direction
}


impl Ball {
    pub fn new(window_w: f64, window_h:f64) -> Ball{
        Ball{
            position: Position{ x: window_w/2.0, y: window_h/2.0},
            r: 10.0,
            direction: Direction { x: 3.0, y: 3.0 }
        }
    }


    pub fn reset(&mut self,window_w: f64, window_h:f64){
        self.position.x = window_w/2.0;
        self.position.y = window_h/2.0;
    }

    pub fn update(&mut self){
        self.position.x += self.direction.x;
        self.position.y += self.direction.y;
    }

    pub fn change_x_direction(&mut self){
        self.direction.x *= -1.0;
    }

    pub fn change_y_direction(&mut self){
        self.direction.y *= -1.0;
    }

    pub fn draw<G: Graphics>(&self, context: Context, g: &mut G){
        ellipse(
            [1.0;4],
            [self.position.x,self.position.y,self.r,self.r],
            context.transform,
            g
        );
    }
}
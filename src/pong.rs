use piston_window::{rectangle, Context, Graphics};

use crate::game::Position;

pub struct Pong {
    pub width: f64,
    pub height: f64,
    pub position: Position
}

impl Pong {
    pub fn new(position: Position, width:f64, height: f64)-> Pong{
        Pong{
            width,
            height,
            position
        }
    }
    pub fn move_by(&mut self, offset: f64){
        self.position.y += offset;
    }

    pub fn draw<G: Graphics>(&self, context: Context, graphics: &mut G){
        rectangle(
            [1.0;4],
            [self.position.x, self.position.y,self.width, self.height],
            context.transform,
            graphics
        )
    }
}



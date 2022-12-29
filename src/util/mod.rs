use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct Vec2D {
    pub x: i32,
    pub y: i32,
}
impl Vec2D {
    pub fn flatten(&self, dims: &Vec2D) -> Option<i32> {
        if &self.x >= &dims.x || &self.y >= &dims.y || &self.x < &0 || &self.y < &0 {
            return None;
        }
        Some((&self.y + (&self.x * dims.y)).into())
    }

    pub fn flat(x: i32, y: i32, dims: &Vec2D) -> Option<i32> {
        if &x >= &dims.x || &y >= &dims.y || &x < &0 || &y < &0 {
            return None;
        }
        Some((&y + (&x * dims.y)).into())
    }

    pub fn copy_expand(&self, width: i32, height: i32) -> Self {
        Vec2D {
            x: &self.x * width,
            y: &self.y * height,
        }
    }

    pub fn from_tuple(i: (i32, i32)) -> Self {
        Vec2D { x: i.0, y: i.1 }
    }
}

pub fn num_to_digit_coords(n: u8) {
    // Gross and hardcoded for now, mostly just a helper function

    match n {
        0x00 => {}
        0x01 => {}
        _ => {}
    }
}
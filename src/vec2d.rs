use std::fmt;
use std::ops;

#[derive(Clone, Copy, Debug)]
pub struct Vec2d {
    pub x: f32,
    pub y: f32,
}

impl Vec2d {
    pub fn new(x: f32, y: f32) -> Vec2d {
        Vec2d { x, y }
    }

    pub fn from_rotation(rotation: f32) -> Vec2d {
        Vec2d {
            x: rotation.cos(),
            y: rotation.sin(),
        }
    }

    pub fn len(&self) -> f32 {
        (self.x.powf(2.) + self.y.powf(2.)).sqrt()
    }

    pub fn normalize(self) -> Vec2d {
        let distance = self.len();
        if distance != 0. {
            self / distance
        } else {
            self
        }
    }

    pub fn rotation(&self) -> f32 {
        self.y.atan2(self.x)
    }
}

impl ops::Add for Vec2d {
    type Output = Self;

    fn add(self, vec: Self) -> Self {
        Self {
            x: self.x + vec.x,
            y: self.y + vec.y,
        }
    }
}

impl ops::AddAssign for Vec2d {
    fn add_assign(&mut self, vec: Self) {
        self.x += vec.x;
        self.y += vec.y;
    }
}

impl ops::Sub for Vec2d {
    type Output = Self;

    fn sub(self, vec: Self) -> Self {
        Self {
            x: self.x - vec.x,
            y: self.y - vec.y,
        }
    }
}

impl ops::SubAssign for Vec2d {
    fn sub_assign(&mut self, vec: Self) {
        self.x -= vec.x;
        self.y -= vec.y;
    }
}

impl ops::Mul<f32> for Vec2d {
    type Output = Self;

    fn mul(self, factor: f32) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}

impl ops::Div<f32> for Vec2d {
    type Output = Self;

    fn div(self, factor: f32) -> Self {
        Self {
            x: self.x / factor,
            y: self.y / factor,
        }
    }
}

impl ops::DivAssign<f32> for Vec2d {
    fn div_assign(&mut self, factor: f32) {
        self.x /= factor;
        self.y /= factor;
    }
}

impl ops::Div<Vec2d> for f32 {
    type Output = Vec2d;

    fn div(self, vec: Vec2d) -> Vec2d {
        let x = if vec.x == 0. { vec.x } else { self / vec.x };
        let y = if vec.y == 0. { vec.y } else { self / vec.y };
        Vec2d { x, y }
    }
}

impl fmt::Display for Vec2d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.x as u16, self.y as u16)?;
        Ok(())
    }
}

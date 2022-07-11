use std::fmt::{Display, Formatter, Pointer};
use std::ops;
use super::*;

/// 2 dimensional vector.
#[derive(Clone, Debug, Default)]
pub struct Vec2 {
    /// `x` coordinate of the vector
    pub x: f64,
    /// `y` coordinate of the vector
    pub y: f64,
}

impl Display for Vec2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}

impl ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: rhs.x + self.x,
            y: rhs.y + self.y,
        }
    }
}

impl ops::Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::Div<f64> for Vec2 {
    type Output = Vec2;
    fn div(self, rhs: f64) -> Self::Output {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Vec2 {
    pub fn len(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }
    //https://ru.onlinemschool.com/math/library/vector/angl/
    pub fn angle(&self) -> f64 {
        let other_x = 1.0;
        let other_y = 0.0;
        let top = self.x * other_x + self.y * other_y;
        let res = top / self.len();
        println!("({}), top :{}, {}, {}", self, top, res, res.acos());
        if self.y < 0.0 { -res.acos() } else { res.acos() }
    }
    pub fn distance(&self, other: &Vec2) -> f64 {
        ((self.x - other.x).powf(2.0) + (self.y - other.y).powf(2.0)).sqrt()
    }
}

impl trans::Trans for Vec2 {
    fn write_to(&self, writer: &mut dyn std::io::Write) -> std::io::Result<()> {
        self.x.write_to(writer)?;
        self.y.write_to(writer)?;
        Ok(())
    }
    fn read_from(reader: &mut dyn std::io::Read) -> std::io::Result<Self> {
        let x: f64 = trans::Trans::read_from(reader)?;
        let y: f64 = trans::Trans::read_from(reader)?;
        Ok(Self {
            x,
            y,
        })
    }
}
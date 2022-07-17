use super::*;

/// RGBA Color
#[derive(Clone, Debug, Copy)]
pub struct Color {
    /// Red component
    pub r: f64,
    /// Green component
    pub g: f64,
    /// Blue component
    pub b: f64,
    /// Alpha (opacity) component
    pub a: f64,
}

pub static RED: Color = Color {
    r: 255.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};

pub static TRANSPARENT_BLACK: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 0.5,
};
pub static TRANSPARENT_ORANGE: Color = Color {
    r: 255.0,
    g: 165.0,
    b: 0.0,
    a: 0.5,
};

pub static YELLOW: Color = Color {
    r: 255.0,
    g: 165.0,
    b: 0.0,
    a: 1.0,
};
pub static GREEN: Color = Color {
    r: 0.0,
    g: 255.0,
    b: 0.0,
    a: 1.0,
};
pub static TRANSPARENT_GREEN: Color = Color {
    r: 0.0,
    g: 255.0,
    b: 0.0,
    a: 0.3,
};
pub static BLUE: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 255.0,
    a: 1.0,
};
pub static TRANSPARENT_BLUE: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 255.0,
    a: 0.3,
};
pub static TEAL: Color = Color {
    r: 0.0,
    g: 255.0,
    b: 255.0,
    a: 1.0,
};
pub static TRANSPARENT_TEAL: Color = Color {
    r: 0.0,
    g: 255.0,
    b: 255.0,
    a: 0.5,
};

impl trans::Trans for Color {
    fn write_to(&self, writer: &mut dyn std::io::Write) -> std::io::Result<()> {
        self.r.write_to(writer)?;
        self.g.write_to(writer)?;
        self.b.write_to(writer)?;
        self.a.write_to(writer)?;
        Ok(())
    }
    fn read_from(reader: &mut dyn std::io::Read) -> std::io::Result<Self> {
        let r: f64 = trans::Trans::read_from(reader)?;
        let g: f64 = trans::Trans::read_from(reader)?;
        let b: f64 = trans::Trans::read_from(reader)?;
        let a: f64 = trans::Trans::read_from(reader)?;
        Ok(Self { r, g, b, a })
    }
}

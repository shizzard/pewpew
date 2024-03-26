use bevy::render::color::Color;

pub struct ColorHex(u32);
impl ColorHex {
    pub fn new(c: u32) -> Self {
        Self(c)
    }

    pub fn as_rgba(&self) -> Color {
        let r = ((self.0 >> 16) & 0xFF) as f32 / 255.0;
        let g = ((self.0 >> 8) & 0xFF) as f32 / 255.0;
        let b = (self.0 & 0xFF) as f32 / 255.0;
        Color::rgb(r, g, b)
    }
}

impl Into<Color> for ColorHex {
    fn into(self) -> Color {
        self.as_rgba()
    }
}

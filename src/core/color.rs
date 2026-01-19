use std::u8;

#[derive(Clone, Copy, Debug, Default)]
pub struct Color { 
  pub r: f32,
  pub g: f32,
  pub b: f32,
}

impl Color {
  pub fn new(r: f32, g: f32, b: f32) -> Color {
    Color { r, g, b }
  }
  
  pub fn u8(&self) -> (u8, u8, u8) {
    let r = self.r.clamp(0.0, 1.0);
    let g = self.g.clamp(0.0, 1.0);
    let b = self.b.clamp(0.0, 1.0);

    (
      (r * 255.0).floor() as u8,
      (g * 255.0).floor() as u8,
      (b * 255.0).floor() as u8,
    )
  }
}
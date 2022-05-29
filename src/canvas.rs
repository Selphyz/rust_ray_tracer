use std::ops::{Add, Mul, Sub};
use crate::utils::float_fuzzy_eq;

#[derive(Debug, Clone)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64
}
impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Color {red, green, blue}
    }
    pub fn black() -> Self {Color::new(0.0, 0.0, 0.0)}
}
impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        float_fuzzy_eq(self.red, other.red)
        && float_fuzzy_eq(self.green, other.green)
        && float_fuzzy_eq(self.blue, other.blue)
    }
}
impl Add for Color {
    type Output = Color;
    fn add(self, rhs: Self) -> Self::Output {
        Color::new(self.red + rhs.red, self.green + rhs.green, self.blue + rhs.blue)
    }
}
impl Sub for Color {
    type Output = Color;
    fn sub(self, rhs: Self) -> Self::Output {
        Color::new(self.red - rhs.red, self.green - rhs.green, self.blue - rhs.blue)
    }
}
impl Mul<f64> for Color {
    type Output = Color;
    fn mul(self, rhs: f64) -> Self::Output {
        Color::new(self.red*rhs, self.green*rhs, self.blue*rhs)
    }
}
impl Mul<Color> for Color {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(self.red*rhs.red, self.green*rhs.green, self.blue*rhs.blue)
    }
}
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Color>,
}
impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self{width, height, pixels: vec![Color::black();  width*height]}
    }
    pub fn pixel_at(&self, x: usize, y: usize) -> &Color{
        &self.pixels[self.get_pixel_index(x, y)]
    }
    pub fn write_color(&mut self, x: usize, y: usize, color: Color) {
        let index = self.get_pixel_index(x ,y);
        self.pixels[index] = color
    }
    pub fn get_pixel_index(&self, x: usize, y: usize) -> usize{
        y*self.width+x
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn colors_are_rbg() {
        let (red, green, blue) = (-0.5, 0.4, 1.7);
        let color = Color::new(red, green, blue);
        assert_eq!(color.red, red);
        assert_eq!(color.green, green);
        assert_eq!(color.blue, blue);
    }
    #[test]
    fn adding_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let expected = Color::new(1.6, 0.7, 1.0);
        let result = c1 + c2;
        assert_eq!(expected, result);
    }
    #[test]
    fn substracting_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let expected = Color::new(0.2, 0.5, 0.5);
        let result = c1 - c2;
        assert_eq!(expected, result);
    }
    #[test]
    fn multiply_color_scalar() {
        let c1 = Color::new(0.2, 0.3, 0.4);
        let expected = Color::new(0.4, 0.6, 0.8);
        let result = c1 * 2.0;
        assert_eq!(expected, result);
    }
    #[test]
    fn multiply_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        let expected = Color::new(0.9, 0.2, 0.04);
        let result = c1*c2;
        assert_eq!(expected, result);
    }
    #[test]
    fn creating_canvas() {
        let c = Canvas::new(10, 20);
        assert_eq!(10, c.width);
        assert_eq!(20, c.height);
        for x in 0..c.width -1{
            for y in  0..c.height -1{
                assert_eq!(*c.pixel_at(x, y), Color::black())
            }
        }
    }
    #[test]
    fn writing_pixels_to_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);
        c.write_color(2, 3, red);
        let expected = Color::new(1.0, 0.0, 0.0);
        assert_eq!(expected, *c.pixel_at(2, 3));
    }
}

use std::ops::{Add, Mul, Sub};
use crate::utils::float_fuzzy_eq;

#[derive(Debug)]
struct Color {
    red: f64,
    green: f64,
    blue: f64
}
impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Color {red, green, blue}
    }
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
}

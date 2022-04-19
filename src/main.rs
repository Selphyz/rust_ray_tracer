fn float_fuzzy_eq(left: f64, right: f64) -> bool {
    let eps = 0.001;
    (left-right).abs() > eps
}
fn main() {
    println!("Hello, world!");
}
use std::ops;
impl ops::Add<Self> for Points {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Points{x: self.x+rhs.x, y: self.y+rhs.y, z: self.z+rhs.z, w: self.w+rhs.w}
    }
}
#[derive(Copy, Clone, Debug, PartialEq)]
struct Points {
    x: f64,
    y: f64,
    z: f64,
    w: f64
}
impl Points{
    pub fn vertex(x: f64, y: f64, z: f64) -> Self{
        Self {x, y, z, w: 1.0 }
    }
    pub fn vector(x: f64, y: f64, z: f64) -> Self{
        Self {x, y, z, w: 0.0 }
    }
    pub fn is_vertex(&self) -> bool {
        self.w==1.0
    }
    pub fn is_vector(&self) -> bool {
        self.w==0.0
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_point_can_be_created() {
        let point = Points::vertex(2.2, -3.1, 3.1);
        assert_eq!(point.x, 2.2);
        assert_eq!(point.y, -3.1);
        assert_eq!(point.z, 3.1);
        assert_eq!(point.w, 1.0);
    }
    #[test]
    fn test_vector_can_be_created() {
        let vector = Points::vector(4.3, -4.2, 3.1);
        assert_eq!(vector.x, 4.3);
        assert_eq!(vector.y, -4.2);
        assert_eq!(vector.z, 3.1);
        assert_eq!(vector.w, 0.0);
    }
    #[test]
    fn test_is_point(){
        let point = Points::vertex(2.7, 5.2, 9.9);
        assert!(point.is_vertex())
    }
    #[test]
    fn test_is_vertex(){
        let vertex = Points::vector(2.3, 4.2, 7.9);
        assert!(vertex.is_vector())
    }
    #[test]
    fn points_combined(){
        let point_one = Points{x: 2.7, y: -5.2, z: 9.9, w: 0.0};
        let point_two = Points{x: -2.7, y: 5.2, z: -9.9, w: 1.0};
        let expected = Points{x: 0.0, y: 0.0, z: 0.0, w: 1.0};
        assert_eq!(point_one+point_two, expected)
    }
}
use std::ops;
use crate::utils::float_fuzzy_eq;

impl ops::Add<Self> for Points {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Points{x: self.x+rhs.x, y: self.y+rhs.y, z: self.z+rhs.z, w: self.w+rhs.w}
    }
}
impl ops::Sub<Self> for Points {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Points{x: self.x-rhs.x, y: self.y-rhs.y, z: self.z-rhs.z, w: self.w-rhs.w}
    }
}
impl ops::Neg for Points {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Points::new(-self.x, -self.y, -self.z, -self.w)
    }
}
impl ops::Mul<f64> for Points {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Points::new(self.x*rhs, self.y*rhs, self.z*rhs, self.w*rhs)
    }
}
impl ops::Div<f64> for Points {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Points::new(self.x/rhs, self.y/rhs, self.z/rhs, self.w/rhs)
    }
}
#[derive(Copy, Clone, Debug)]
pub struct Points {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}
impl PartialEq<Points> for Points {
    fn eq(&self, other: &Self) -> bool {
        float_fuzzy_eq(self.x, other.x)
            && float_fuzzy_eq(self.y, other.y)
            && float_fuzzy_eq(self.z, other.z)
            && float_fuzzy_eq(self.w, other.w)
    }
}
impl Points{
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self { Self { x, y, z, w }}
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
    pub fn magnitude(&self) -> f64 {(self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()}
    pub fn normalize(&self) -> Self { *self / self.magnitude()}
    pub fn dot(&self, other: &Points) -> f64 { self.x*other.x + self.y*other.y + self.z*other.z + self.w*other.w }
    pub fn cross(&self, other: &Points) -> Points {
        if !self.is_vector() || !other.is_vector(){
            panic!("Cross product must be from two vectors")
        }
        Points::vector(self.y*other.z - self.z*other.y, self.z*other.x - self.x*other.z, self.x*other.y - self.y*other.x,)
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
    fn test_points_combined(){
        let point_one = Points::new(2.7, -5.2, 9.9, 0.0);
        let point_two = Points::new(-2.7, 5.2, -9.9, 1.0);
        let expected = Points::new(0.0, 0.0, 0.0, 1.0);
        let result = point_one + point_two;
        assert_eq!(expected, result)
    }
    #[test]
    fn test_points_diff(){
        let point_one = Points::vertex(2.7, -5.2, 9.9);
        let point_two = Points::vertex(-2.7, 5.2, -9.9);
        let expected = Points::vector(5.4, -10.4, 19.8);
        let result = point_one-point_two;
        assert_eq!(result, expected);
        assert!(result.is_vector());
    }
    #[test]
    fn test_vec_point_diff(){
        let point = Points::vertex(2.7, -5.2, 9.9);
        let vec = Points::vector(2.7, -5.2, 9.9);
        let expected = Points::vertex(0.0, 0.0, 0.0);
        let result = point-vec;
        assert!(result.is_vertex());
        assert_eq!(expected, result);
    }
    #[test]
    fn test_vec_diff(){
        let vec1 = Points::vector(2.7, -5.2, 9.9);
        let vec2 = Points::vector(2.7, -5.2, 9.9);
        let expected = vec1-vec2;
        let result = Points::vector(0.0, 0.0, 0.0);
        assert!(expected.is_vector());
        assert_eq!(expected, result);
    }
    #[test]
    fn diff_zero_vector(){
        let vec1 = Points::vector(0.0, 0.0, 0.0);
        let vec2 = Points::vector(2.7, 5.2, 9.9);
        let expected = vec1-vec2;
        let result = Points::vector(-2.7, -5.2, -9.9);
        assert!(expected.is_vector());
        assert_eq!(expected, result);
    }
    #[test]
    fn negate_tuple() {
        let point = Points::new(1.0, 2.0, 3.0, -4.0);
        let expected = Points::new(-1.0, -2.0, -3.0, 4.0);
        let result = -point;
        assert_eq!(expected, result);
    }
    #[test]
    fn multiply_tuple_by_scalar() {
        let point = Points::new(1.0, 2.0, 3.0, -4.0);
        let mult: f64 = 3.5;
        let expected = Points::new(3.5, 7.0, 10.5, -14.0);
        let result = point * mult;
        assert_eq!(expected, result);
    }
    #[test]
    fn multiply_tuple_by_fraction() {
        let point = Points::new(1.0, 2.0, 3.0, -4.0);
        let mult = 0.5;
        let expected = Points::new(0.5, 1.0, 1.5, -2.0);
        let result = point * mult;
        assert_eq!(expected, result);
    }
    #[test]
    fn divide_tuple_by_scalar() {
        let point = Points::new(1.0, 2.0, 3.0, -4.0);
        let mult = 2.0;
        let expected = Points::new(0.5, 1.0, 1.5, -2.0);
        let result = point / mult;
        assert_eq!(expected, result);
    }
    #[test]
    fn compute_magnitude_of_vector_1_0_0() {
        let vector = Points::new(1.0, 0.0, 0.0, 0.0);
        let expected = 1.0;
        let result = vector.magnitude();
        assert_eq!(expected, result);
    }
    #[test]
    fn compute_magnitude_of_vector_0_1_0() {
        let vector = Points::new(0.0, 1.0, 0.0, 0.0);
        let expected = 1.0;
        let result = vector.magnitude();
        assert_eq!(expected, result);
    }
    #[test]
    fn compute_magnitude_of_vector_0_0_1() {
        let vector = Points::new(0.0, 0.0, 1.0, 0.0);
        let expected = 1.0;
        let result = vector.magnitude();
        assert_eq!(expected, result);
    }
    #[test]
    fn compute_magnitude_of_vector_1_2_3() {
        let vector = Points::new(1.0, 2.0, 3.0, 0.0);
        let expected = (14.0f64).sqrt();
        let result = vector.magnitude();
        assert_eq!(expected, result);
    }
    #[test]
    fn compute_magnitude_of_neg_vector_1_2_3() {
        let vector = Points::new(-1.0, -2.0, -3.0, 0.0);
        let expected = (14.0f64).sqrt();
        let result = vector.magnitude();
        assert_eq!(expected, result);
    }
    #[test]
    fn normalize_vector() {
        let vector = Points::vector(4.0, 0.0, 0.0);
        let expected = Points::vector(1.0, 0.0, 0.0);
        let result = vector.normalize();
        assert_eq!(expected, result);
    }
    #[test]
    fn normalize_vector_1_2_3() {
        let vector = Points::vector(1.0, 2.0, 3.0);
        let expected = Points::vector(0.26726, 0.53452, 0.80178);
        let result = vector.normalize();
        assert_eq!(expected, result);
    }
    #[test]
    fn normalized_vector_magnitude() {
        let vector = Points::vector(1.0, 2.0, 3.0);
        let expected = 1.0;
        let result = vector.normalize().magnitude();
        assert_eq!(expected, result);
    }
    #[test]
    fn dot_product_of_points() {
        let a = Points::vector(1.0, 2.0, 3.0);
        let b = Points::vector(2.0, 3.0, 4.0);
        let expected = 20.0;
        let result = a.dot(&b);
        assert_eq!(expected, result);
    }
    #[test]
    fn cross_product_of_vectors_a_b() {
        let a = Points::vector(1.0, 2.0, 3.0);
        let b = Points::vector(2.0, 3.0, 4.0);
        let expected = Points::vector(-1.0, 2.0, -1.0);
        let result = a.cross(&b);
        assert_eq!(expected, result);
    }
    #[test]
    fn cross_product_of_vectors_b_a() {
        let a = Points::vector(1.0, 2.0, 3.0);
        let b = Points::vector(2.0, 3.0, 4.0);
        let expected = Points::vector(1.0, -2.0, 1.0);
        let result = b.cross(&a);
        assert_eq!(expected, result);
    }
}

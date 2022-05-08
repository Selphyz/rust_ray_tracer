mod points;
mod utils;

use crate::points::Points;

fn main() {
    let env = Environment::new(Points::vector(0.0, -0.1, 0.0), Points::vector(0.0001, 0.0, 0.0));
    let proyectile = Proyectile::new(Points::vector(0.0, 1.0, 0.0), Points::vector(0.02, 0.0, 0.0));
    let mut current = proyectile;
    let mut iteration = 0;
    while current.position.y > 0.0 {
        println!("{}: {:?}", iteration, &current);
        current = tick(&env, &current);
        iteration+=1;
    }
    println!("END => {}: {:?}", iteration, &current);
}
struct Environment {
    gravity: Points,
    wind: Points
}
#[derive(Debug)]
struct Proyectile {
    position: Points,
    velocity: Points
}
impl Proyectile {
    pub fn new(position: Points, velocity: Points) -> Self {
        Proyectile { position, velocity}
    }
}
impl Environment {
    pub fn new(gravity: Points, wind: Points) -> Self {
        Environment { gravity, wind}
    }
}
fn tick(env: &Environment, proyectile: &Proyectile) -> Proyectile{
    Proyectile::new(proyectile.position + proyectile.velocity, proyectile.velocity + env.gravity + env.wind)
}

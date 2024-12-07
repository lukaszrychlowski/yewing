use yew::prelude::*;
use rand::{thread_rng, Rng};
use std::iter;

struct Vector2D {
   x: f64,
   y: f64
}

impl Vector2D {
    fn new(x: f64, y: f64) -> Self {
	Self { x, y }
    }
}

struct Particle {
   position: Vector2D,
   velocity: Vector2D,
   radius: f64,
   hue: f64
}

impl Particle {
    fn new() -> Self {
	let mut rng = rand::thread_rng();
	Self {
	    position: Vector2D::new(rng.gen::<f64>(), rng.gen::<f64>()),
	    velocity: Vector2D::new(rng.gen::<f64>(), rng.gen::<f64>()),
	    radius: rng.gen::<f64>(),
	    hue: rng.gen::<f64>()
	}
    }
    fn generate_particles() -> Vec<Particle> {
	let mut vec = Vec::new();
	let no_of_particles = 100;
	for i in 0..no_of_particles {
	    vec.push(Particle::new());
	}
	vec
    }
	
    fn draw(&self) -> Html {
	let x = format!("{}", self.position.x * 1080.0); // * innerWidth()
	let y = format!("{}", self.position.y * 1600.0); // * innerHeight()
	let radius = format!("{}", self.radius * 20.0);
	html! {
	    <circle cx={x} cy={y} r={radius} fill="#aede" stroke="black"/>
	}
    }
}

#[function_component]
fn App() -> Html {
    let particles = Particle::generate_particles();
    html! {
	<svg width="100vw" height="100vh">
	   { for particles.iter().map(|particle| particle.draw()) }
        </svg>
    }  
}

fn main() {
    yew::Renderer::<App>::new().render();
}

use yew::prelude::*;
use rand::{thread_rng, Rng};
use std::iter;
use gloo::timers::callback::Interval;

#[derive(Clone)]
struct Vector2D {
   x: f64,
   y: f64
}

impl Vector2D {
    fn new(x: f64, y: f64) -> Self {
	Self { x, y }
    }
}
#[derive(Clone)]
struct Particle {
   position: Vector2D, // [m]
   velocity: Vector2D, //[m/s]
   acceleration: Vector2D, //[m/s2]
   radius: f64, // [m]
   hue: f64 
}


impl Particle {
    fn new() -> Self {
	let mut rng = rand::thread_rng();
	Self {
	    position: Vector2D::new(rng.gen::<f64>(), rng.gen::<f64>()),
	    acceleration: Vector2D::new(0.0, 1.0 / 9.81 / 62.5),
	    velocity: Vector2D::new(0.001 * rng.gen::<f64>(), 0.001 * rng.gen::<f64>()),
	    radius: rng.gen::<f64>(),
	    hue: rng.gen::<f64>()
	}
    }
    fn generate_particles(no_of_particles: i32) -> Vec<Particle> {
	(0..no_of_particles).map(|_| Particle::new()).collect()
    }
	
    fn draw(&self) -> Html {
	let x = format!("{}", self.position.x * 1080.0); // * innerWidth()
	let y = format!("{}", self.position.y * 1565.0); // * innerHeight()
	let radius = format!("{}", self.radius * 10.0);
	html! {
	    <circle cx={x} cy={y} r={radius} fill="#aede" stroke="black"/>
	}
    }

    fn update(&mut self) {
	self.velocity.x += self.acceleration.x;
	self.velocity.y += self.acceleration.y;
	
	self.position.x +=  self.velocity.x;
	self.position.y +=  self.velocity.y;

	if self.position.x < 0.0 || self.position.x > 1.0 {
	    self.velocity.x = -self.velocity.x;
	}
	if self.position.y < 0.0 || self.position.y > 1.0 {
	    self.velocity.y = -self.velocity.y;
	}
    }
}

#[function_component]
fn App() -> Html {
    let particles = use_state(|| Particle::generate_particles(1000)); //state of particles is of interest
    {
	let particles = particles.clone();
	use_effect(|| {
	    let interval = Interval::new(16, move || { //1sec interval
		particles.set({
		    let mut updated_particles = (*particles).clone();
		    for particle in &mut updated_particles {
			particle.update();
		    }
		    updated_particles
		});
	    });
	    move || drop(interval)
	});
    }
    
    html! {
	<svg width="100vw" height="100vh" viewbox="0 0 100 100" >
	   { for particles.iter().map(|particle| particle.draw()) }
        </svg>
    }  
}

fn main() {
    yew::Renderer::<App>::new().render();
}

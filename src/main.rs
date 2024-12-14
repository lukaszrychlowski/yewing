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
    fn generate_particles(no_of_particles: i32) -> Vec<Particle> {
	(0..no_of_particles).map(|_| Particle::new()).collect()
    }
	
    fn draw(&self) -> Html {
	let x = format!("{}", self.position.x * 1080.0); // * innerWidth()
	let y = format!("{}", self.position.y * 2000.0); // * innerHeight()
	let radius = format!("{}", self.radius * 10.0);
	html! {
	    <circle cx={x} cy={y} r={radius} fill="#aede" stroke="black"/>
	}
    }

    fn update(&mut self) {
	self.position.x += self.velocity.x/1600.0;
	self.position.y += self.velocity.y/1600.0;

	if self.position.x < 0.0 || self.position.x > 1.0 {
	    self.velocity.x = -self.velocity.x
	}
	if self.position.y < 0.0 || self.position.y > 1.0 {
	    self.velocity.y = -self.velocity.y
	}
    }
}

#[function_component]
fn App() -> Html {
    let particles = use_state(|| Particle::generate_particles(1000));
    {
	let particles = particles.clone();
	use_effect(move || {
	    let interval = Interval::new(16, move || {
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
	<svg width="100vw" height="100vh">
	   { for particles.iter().map(|particle| particle.draw()) }
        </svg>
    }  
}

fn main() {
    yew::Renderer::<App>::new().render();
}

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
	    //position: Vector2D::new(0.50, 0.0),
	    acceleration: Vector2D::new(0.0, 0.0),
	    velocity: Vector2D::new(rng.gen::<f64>(), rng.gen::<f64>()), //0 - 1
	    //velocity: Vector2D::new(0.0, 0.0),
	    radius: rng.gen::<f64>(),
	    hue: rng.gen::<f64>()
	}
    }
    
    fn generate_particles(no_of_particles: i32) -> Vec<Particle> {
	(0..no_of_particles).map(|_| Particle::new()).collect()
    }
	
    fn draw(&self) -> Html {
	let x = format!("{}", self.position.x * 1000.0); // * innerWidth()
	let y = format!("{}", self.position.y * 1000.0); // * innerHeight()
	let radius = format!("{}", self.radius*20.0);
	html! {
	    <circle cx={x} cy={y} r={radius} fill="#aede" stroke="black"/>
	}
    }

    fn update_state(&mut self) {
	const GRAVITY: f64 = 9.8;
	const FRICTION_COEFF: f64 = 0.05;
	const RESTITUTION: f64 = 0.3;
	const TIME_STEP: f64 = 0.016;
	
	self.position.x +=  self.velocity.x * TIME_STEP;
	self.position.y +=  self.velocity.y * TIME_STEP;

	self.velocity.x += self.acceleration.x * TIME_STEP;
	self.velocity.y += self.acceleration.y + GRAVITY * TIME_STEP;

	self.velocity.x *= 1.0 - FRICTION_COEFF;
	self.velocity.y *= 1.0 - FRICTION_COEFF;
	
	if self.position.x <= 0.0 || self.position.x >= 1.0 {
	    self.velocity.x = -RESTITUTION * self.velocity.x;
	}
	if self.position.y >= 1.0 {
	    self.velocity.y = -RESTITUTION * self.velocity.y
	}
	if self.velocity.y.abs() < 0.05 {
	    self.velocity.y = 0.0;
	}
	if self.velocity.x.abs() < 0.01 {
	    self.velocity.x = 0.0;
	}
    }
}

#[function_component]
fn App() -> Html {
    let particles = use_state(|| Particle::generate_particles(10000)); //state of particles is of interest
    {
	let particles = particles.clone();
	use_effect(|| {
	    let interval = Interval::new(16, move || {
		particles.set({
		    let mut updated_particles = (*particles).clone();
		    for particle in &mut updated_particles {
			particle.update_state();
		    }
		    updated_particles
		});
	    });
	    move || drop(interval)
	});
    }
    
    html! {
	<svg width="1000" height="1000" viewbox="0 0 100 100" >
	    <rect width="1000" height="1000" fill="none" stroke="black" stroke-width="10"/>
	    <text x="20" y=" 30" class="small"> { particles[0].position.x } </text>
	    <text x="20" y=" 45" class="small"> { particles[0].position.y } </text>
	    <text x="20" y=" 60" class="small"> { particles[0].velocity.x } </text>
	    <text x="20" y=" 75" class="small"> { particles[0].velocity.y } </text>
	    <text x="20" y=" 90" class="small"> { particles[0].acceleration.x } </text>
	    <text x="20" y="105" class="small"> { particles[0].acceleration.y } </text>
	    
	   { for particles.iter().map(|particle| particle.draw()) }
        </svg>
    }  
}

fn main() {
    yew::Renderer::<App>::new().render();
}

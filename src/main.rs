use yew::prelude::*;
use rand::{Rng};
use gloo::timers::callback::Interval;

#[derive(Clone, PartialEq)]
struct Vector2D {
   x: f64,
   y: f64
}

impl Vector2D {
    fn new(x: f64, y: f64) -> Self {
	Self { x, y }
    }
}
#[derive(Clone, PartialEq)]
struct Particle {
   position: Vector2D, // [m]
   velocity: Vector2D, //[m/s]
   acceleration: Vector2D, //[m/s2]
   radius: f64, // [m]
   hue: String,
   collision: bool
}


impl Particle {
    fn new() -> Self {
	let mut rng = rand::thread_rng();
	Self {
	    position: Vector2D::new(rng.gen::<f64>(), rng.gen::<f64>()),
	    acceleration: Vector2D::new(0.0, 0.0),
	    velocity: Vector2D::new(rng.gen::<f64>(), rng.gen::<f64>()), //0 - 1
	    //radius: rng.gen::<f64>(),
	    radius: 1.0,
	    hue: "#aede".to_string(),
	    collision: false
	}
    }
    
    fn generate_particles(no_of_particles: i32) -> Vec<Particle> {
	(0..no_of_particles).map(|_| Particle::new()).collect()
    }
	
    fn draw(&self, color: String) -> Html {
	let x = format!("{}", self.position.x * 1000.0); // * innerWidth()
	let y = format!("{}", self.position.y * 1000.0); // * innerHeight()
	let radius = format!("{}", self.radius * 25.0);
	html! {
	    <circle cx={x} cy={y} r={radius} fill={color} stroke="black"/>
	}
    }
	
    fn update_state(&mut self, collision: bool) {
	const GRAVITY: f64 = 9.8;
	const FRICTION_COEFF: f64 = 0.025;
	const RESTITUTION: f64 = 0.45;
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
	if self.velocity.y.abs() < 0.1 {
	    self.velocity.y = 0.0;
	}
	if self.velocity.x.abs() < 0.01 {
	    self.velocity.x = 0.0;
	}

	self.hue = if collision == true { "#babe".to_string() } else { "#aede".to_string() };
    }

    fn check_collision(&mut self, particles: &[Particle]) -> bool {
	let mut collision = false;
	for particle in particles.iter() {
	    if self == particle {
		log::debug!("self");
		continue;
	   }
	   let dx = 1000.0 * (self.position.x - particle.position.x);
	   let dy = 1000.0 * (self.position.y - particle.position.y);
	   let dist = (dx.powf(2.0) + dy.powf(2.0)).sqrt();
	   if dist <= 25.0 * ( self.radius + particle.radius)
	   {
	       collision = true;
	       break;
	   }
	}
	return collision;
    }
}
	    

#[function_component]
fn App() -> Html {
    const NO_OF_PARTICLES: i32 = 30;
    const INTERVAL: u32 = 200;
    let particles = use_state(|| Particle::generate_particles(NO_OF_PARTICLES)); //state of particles is of interest
    let onclick = {
	let particles = particles.clone();
	Callback::from(move |_| {
	    particles.set(Particle::generate_particles(NO_OF_PARTICLES));
	 })
    };
    
    {
	let particles = particles.clone();
	use_effect(|| {
	    let interval = Interval::new(INTERVAL, move || {
		particles.set({
		    let mut updated_particles = (*particles).clone();
		    let particles_snap = (*particles).clone();
		    for particle in &mut updated_particles {
			//particle.update_state();
			let collision = particle.check_collision(&particles_snap);
			particle.update_state(collision);
		    }
		    updated_particles
		});
	    });
	    move || drop(interval)
	});
    }
  
    html! {
	<>
	    <div>
	    <button {onclick}>{ "reset" }</button>
	    </div>
	<svg width="1000" height="1000" viewbox="0 0 100 100" >
	    <rect width="1000" height="1000" fill="none" stroke="black" stroke-width="10"/>
	    <text x="20" y=" 30" class="small"> { particles[0].position.x } </text>
	    <text x="20" y=" 45" class="small"> { particles[0].position.y } </text>
	    <text x="20" y=" 60" class="small"> { particles[0].velocity.x } </text>
	    <text x="20" y=" 75" class="small"> { particles[0].velocity.y } </text>
	    <text x="20" y=" 90" class="small"> { particles[0].acceleration.x } </text>
	    <text x="20" y="105" class="small"> { particles[0].acceleration.y } </text>
	    <text x="20" y="120" class="small"> { particles[0].radius } </text>
	   { for particles.iter().map(|particle| particle.draw(particle.hue.clone())) }
        </svg>
	    </>
    }  
}

fn main() {
    console_log::init_with_level(log::Level::Debug).expect("Failed to initialize logger");
    yew::Renderer::<App>::new().render();
}

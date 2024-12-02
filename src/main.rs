use yew::prelude::*;
use rand::{thread_rng, Rng};
use std::fmt::Write;

struct Vector2D {
   x: f64,
   y: f64
}

impl Vector2D {
    pub fn new(x: f64, y: f64) -> Self {
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
    pub fn randomize_state() -> Self {
	let mut rng = rand::thread_rng();
	Self {
	    position: Vector2D::new(rng.gen::<f64>(), rng.gen::<f64>()),
	    velocity: Vector2D::new(rng.gen::<f64>(), rng.gen::<f64>()),
	    radius: rng.gen::<f64>(),
	    hue: rng.gen::<f64>()
	}
    }
}

#[function_component]
fn Adder() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
	let counter = counter.clone();
	move |_| {
	    counter.set(*counter + 2);
	}
    };
    
    html! {
	<div>
	    <button {onclick}>{ "+2" }</button>
	    <p>{ *counter }</p>
	</div>
    }
}

#[function_component]
fn Randomizer() -> Html {
    let particle = Particle::randomize_state();
    html! {
	<svg width="760" height="1080" viewBox="-540 -760 760 1080" fill="none">
	    <circle cx="particle.position.x" cy="particle.position.y" r="50" fill="#aede" stroke="black"/>
	</svg>    
    }
}

fn main() {
    yew::Renderer::<Randomizer>::new().render();
}

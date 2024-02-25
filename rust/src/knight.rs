use godot::{
    engine::{CharacterBody3D, ICharacterBody3D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
struct Knight {
    speed: real,

    base: Base<CharacterBody3D>,
}

#[godot_api]
impl ICharacterBody3D for Knight {
    fn init(base: Base<CharacterBody3D>) -> Self {
        Self { speed: 250.0, base }
    }
    fn ready(&mut self) {
        godot_print!("Init knight!");
    }
    fn physics_process(&mut self, delta: f64) {
        let mut direction = Vector3::ZERO;
        let input = Input::singleton();
        if input.is_action_pressed("move_right".into()) {
            godot_print!("right pressed");
            direction.x += 1.0;
        }
        if input.is_action_pressed("move_left".into()) {
            direction.x -= 1.0;
        }
        if input.is_action_pressed("move_down".into()) {
            direction.z += 1.0;
        }
        if input.is_action_pressed("move_up".into()) {
            direction.z -= 1.0;
        }
        if direction != Vector3::ZERO {
            direction = direction.normalized();
            let mut pivot = self.base().get_node_as::<Node3D>("Pivot");
            pivot.set_basis(Basis::new_looking_at(direction, Vector3::UP, true));
        }

        let mut target_velocity = Vector3::ZERO;
        target_velocity.x = direction.x * self.speed * real::from_f64(delta);
        target_velocity.z = direction.z * self.speed * real::from_f64(delta);
        godot_print!("target {}", target_velocity.x);
        // let change = direction * self.speed * real::from_f64(delta);
        self.base_mut().set_velocity(target_velocity);
        self.base_mut().move_and_slide();
    }
}

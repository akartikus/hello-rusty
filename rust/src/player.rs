use godot::engine::{ISprite2D, Sprite2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Player {
    speed: real,
    angular_speed: f64,
    screen_size: Vector2,

    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for Player {
    fn init(base: Base<Sprite2D>) -> Self {
        Self {
            speed: 400.0,
            angular_speed: std::f64::consts::PI,
            screen_size: Vector2::new(0.0, 0.0),

            base,
        }
    }

    fn ready(&mut self) {
        let viewport = self.base().get_viewport_rect();
        self.screen_size = viewport.size;
    }

    fn physics_process(&mut self, delta: f64) {
        let radians = (self.angular_speed * delta) as f32;
        self.base_mut().rotate(radians);
    }

    fn process(&mut self, delta: f64) {
        let mut velocity = Vector2::new(0.0, 0.0);

        let input = Input::singleton();
        if input.is_action_pressed("move_right".into()) {
            velocity += Vector2::RIGHT;
        }
        if input.is_action_pressed("move_left".into()) {
            velocity += Vector2::LEFT;
        }
        if input.is_action_pressed("move_up".into()) {
            velocity += Vector2::UP;
        }
        if input.is_action_pressed("move_down".into()) {
            velocity += Vector2::DOWN;
        }
        if velocity.length() > 0.0 {
            velocity = velocity.normalized() * self.speed;
        }

        let change = velocity * real::from_f64(delta);
        let position = self.base().get_global_position() + change;
        let position = Vector2::new(
            position.x.clamp(0.0, self.screen_size.x),
            position.y.clamp(0.0, self.screen_size.y),
        );

        self.base_mut().set_global_position(position);
    }
}

// #[godot_api]
// impl Player {
//     #[func]
//     fn increase_speed(&mut self, amount: f64) {
//         self.speed += amount;
//         self.base_mut().emit_signal("speed_increased".into(), &[]);
//     }

//     #[signal]
//     fn speed_increased();
// }

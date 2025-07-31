use godot::classes::{AnimatedSprite2D, CharacterBody2D, Node, SpriteFrames};
use godot::prelude::*;
use godot::classes::Input;
use godot::builtin::Vector2;

use crate::SpriteAnimationLoader;
use crate::State_Machine::state::StateLogic;

use crate::State_Machine::car::car_consts::*;

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct CarRightState {
    #[export]
    path_sprite_frames: GString,
    car_sprite_frames: Gd<SpriteFrames>,
    name: String,
    base: Base<Node>,
    car_body_2D: Option<Gd<CharacterBody2D>>,
}

use godot::classes::INode;

#[godot_api]
impl INode for CarRightState {
    fn ready(&mut self) {
        self.name = "CarRightState".to_string();
        self.car_sprite_frames =
            SpriteAnimationLoader::load_sprite_frames(&self.path_sprite_frames.to_string());

        let parent = match self.base().get_parent() {
            Some(parent) => parent,
            None => panic!("pas de parent"),
        };
        self.car_body_2D = match parent.get_parent() {
            Some(parent) => Some(parent.cast::<CharacterBody2D>()) ,
            None => panic!("pas de parent2"),
        };
    }
}

#[godot_api]
impl CarRightState {
    #[signal]
    fn transitionned();
}

impl StateLogic for CarRightState {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn enter(&mut self) {
        println!("aazaz");
        let parent = match self.base().get_parent() {
            Some(parent) => parent,
            None => panic!("pas de parent"),
        };
        let parent = match parent.get_parent() {
            Some(parent) => parent,
            None => panic!("pas de parent2"),
        };

        let mut animator: Gd<AnimatedSprite2D> = parent.get_node_as("AnimatedSprite2D");
        println!("aaaa {animator}");
        SpriteAnimationLoader::set_sprite_frames(&mut animator, &self.car_sprite_frames);
    }

    fn exit(&mut self) {}

    fn update(&mut self, delta: f64) {

        let car_body_2_d = match &mut self.car_body_2D {
            Some(body) => body,
            None => panic!("PAS DE CARACBODY SUR LA VOITURE"),
        };

        let mut velocity: Vector2 = { car_body_2_d.get_velocity() };

        let direction: f32 = Input::singleton().get_axis("ui_left", "ui_right");
        if direction != 0.0 {
            let target_speed = direction * MAX_SPEED as f32;
            velocity.x = godot::global::move_toward(
                velocity.x as f64,
                target_speed as f64,
                (ACCELERATION * delta) as f64,
            ) as f32;
        } else {
             velocity.x = godot::global::move_toward(
        velocity.x as f64,
        0.0,
        (DECELERATION * delta) as f64,
    ) as f32;        }
        car_body_2_d.set_velocity(velocity);

        car_body_2_d.move_and_slide();
    }

    fn physics_update(&mut self, delta: f64) {}

    fn connect(&mut self, singal_name: String, callable: Callable) {}
}

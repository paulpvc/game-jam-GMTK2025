
use godot::builtin::Vector2;
use godot::classes::Input;
use godot::classes::{AnimatedSprite2D, CharacterBody2D, Node, SpriteFrames, Timer};
use godot::prelude::*;

use crate::SpriteAnimationLoader;
use crate::State_Machine::state::StateLogic;

use crate::State_Machine::car::car_consts::*;

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct CarDownRightState {
    #[export]
    path_sprite_frames: GString,
    car_sprite_frames: Gd<SpriteFrames>,
    name: String,
    base: Base<Node>,
    car_body_2D: Option<Gd<CharacterBody2D>>,
    timer: Option<Gd<Timer>>,
    timer_started: bool,
}

use godot::classes::INode;

#[godot_api]
impl INode for CarDownRightState {
    fn ready(&mut self) {
        self.name = "CarDownRightState".to_string();
        self.car_sprite_frames =
            SpriteAnimationLoader::load_sprite_frames(&self.path_sprite_frames.to_string());

        let parent = match self.base().get_parent() {
            Some(parent) => parent,
            None => panic!("pas de parent"),
        };
        self.car_body_2D = match parent.get_parent() {
            Some(parent) => Some(parent.cast::<CharacterBody2D>()),
            None => panic!("pas de parent2"),
        };
        let mut timer: Gd<Timer> = self
            .base()
            .get_parent()
            .unwrap()
            .get_parent()
            .unwrap()
            .get_node_as("Timer");
        let callable = self.base_mut().callable("on_timer_timeout");
        timer.connect(&"timeout".to_string(), &callable);

        self.timer = Some(timer);
        self.timer_started = false;
    }
}

#[godot_api]
impl CarDownRightState {
    #[signal]
    fn transitionned(new_state: String);
    #[func]
    fn on_timer_timeout(&mut self) {
        if let Some(body) = &self.car_body_2D {
            if body.get_velocity().length() >= TURN_THRESHOLD as f32 {
                let direction_rotation: f32 = Input::singleton().get_axis("ui_up", "ui_down");
                if direction_rotation < 0.0 {
                    godot_print!("tourne vers la gauche");
                    self.base_mut().emit_signal(
                        &StringName::from("transitionned"),
                        &["CarDownRightState".to_variant(), "CarRightState".to_variant()],
                    );
                } else if direction_rotation > 0.0 {
                    self.base_mut().emit_signal(
                        &StringName::from("transitionned"),
                        &["CarDownRightState".to_variant(), "CarDownState".to_variant()],
                    );
//self.base_mut().emit_signal(&StringName::from("transitionned"), &["CarDownRightState".to_variant()]);
                }
            }
        }
        self.timer_started = false;
    }
}

impl StateLogic for CarDownRightState {
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

    fn update(&mut self, delta: f64) {}

    fn physics_update(&mut self, delta: f64) {
        
        let car_body_2_d = match &mut self.car_body_2D {
            Some(body) => body,
            None => panic!("PAS DE CARACBODY SUR LA VOITURE"),
        };

        let mut velocity: Vector2 = { car_body_2_d.get_velocity() };
        //godot_print!("{velocity}");

        let direction_rotation: f32 = Input::singleton().get_axis("ui_up", "ui_down");
        if velocity.length() >= TURN_THRESHOLD as f32 && direction_rotation != 0.0 && !self.timer_started {
            self.timer_started = true;
            self.timer.clone().unwrap().start();
        }

        let direction: f32 = Input::singleton().get_axis("ui_left", "ui_right");
        if direction != 0.0 {
            let target_speed = direction * MAX_SPEED as f32;
            let forward = Vector2::new(1.0, 1.0).normalized();
            velocity = velocity.move_toward(forward * target_speed, (ACCELERATION * delta) as f32);
        } else {
            velocity = velocity.move_toward(Vector2::ZERO, (DECELERATION * delta) as f32);        }
        car_body_2_d.set_velocity(velocity);

        car_body_2_d.move_and_slide();
    }

    fn connect(&mut self, singal_name: &String, callable: Callable) {
        let str_name = StringName::from(singal_name);
        self.base_mut().connect(&str_name, &callable);
        godot_print!("connected {singal_name} to {callable}")
    }
}




use godot::builtin::Vector2;
use godot::classes::{AnimatedSprite2D, CharacterBody2D, Node, SpriteFrames, Timer};
use godot::prelude::*;

use crate::SpriteAnimationLoader;
use crate::State_Machine::state::StateLogic;

use crate::State_Machine::car::car_consts::*;

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct CarDownLeftState {
    #[export]
    path_sprite_frames: GString,
    car_sprite_frames: Gd<SpriteFrames>,
    name: String,
    base: Base<Node>,
    car_body_2D: Option<Gd<CharacterBody2D>>,
    
    facing_direction: Vector2, // Direction vers laquelle la voiture regarde
    is_reversing: bool,
}

use godot::classes::INode;

#[godot_api]
impl INode for CarDownLeftState {
    fn ready(&mut self) {
        self.name = "CarDownLeftState".to_string();
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
        

        

        self.facing_direction = Vector2::new(-1.0, 1.0);
        self.is_reversing = false;
    }
}

#[godot_api]
impl CarDownLeftState {
    #[signal]
    fn transitionned(new_state: String);
    
    fn update_state_from_velocity(&mut self) -> String {
        if let Some(car_body) = &self.car_body_2D {
            let velocity = car_body.get_velocity();

            // Si la vitesse est trop faible, on garde l'état actuel
            if velocity.length() < TURN_THRESHOLD as f32 {
                return self.name();
            }

            // Déterminer si on avance ou on recule
            let dot_product = velocity.dot(self.facing_direction);
            self.is_reversing = dot_product < 0.0;

            let new_state = if self.is_reversing {
                // En marche arrière, on garde la direction du regard
                get_state_from_direction(-velocity)
            } else {
                // En marche avant, on met à jour la direction du regard
                //self.facing_direction = velocity.normalized();
                get_state_from_direction(velocity)
            };
            new_state
            //if new_state != self.name() {

            //}
        } else {
            String::new()
        }
    }

    #[func]
    fn emit_transitionned_signal(&mut self, name: String, new_state_name: String) {
        self.base_mut().emit_signal(
            &StringName::from("transitionned"),
            &[name.to_variant(), new_state_name.to_variant()],
        );
    }
}

impl StateLogic for CarDownLeftState {
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
        let new_state_name = self.update_state_from_velocity();
        let name = self.name();

        if new_state_name != self.name() {
            self.base_mut().call_deferred(
                &StringName::from("emit_transitionned_signal"),
                &[name.to_variant(), new_state_name.to_variant()],
            );
        }

        let car_body_2_d = match &mut self.car_body_2D {
            Some(body) => body,
            None => panic!("PAS DE CARACBODY SUR LA VOITURE"),
        };

        let mut velocity: Vector2 = { car_body_2_d.get_velocity() };
        
        velocity = update_velocity(self.facing_direction, velocity, &delta);
        car_body_2_d.set_velocity(velocity);

        car_body_2_d.move_and_slide();
    }

    fn connect(&mut self, singal_name: &String, callable: Callable) {
        let str_name = StringName::from(singal_name);
        self.base_mut().connect(&str_name, &callable);
        godot_print!("connected {singal_name} to {callable}")
    }
}

use godot::prelude::*;
use godot::classes::{AnimatedSprite2D, Node, SpriteFrames};

use crate::SpriteAnimationLoader;
use crate::State_Machine::state::{StateLogic};



#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct CarRightState {
    #[export]
    path_sprite_frames: GString,
    car_sprite_frames: Gd<SpriteFrames>,
    name: String,
    base: Base<Node>,
}

use godot::classes::INode;

#[godot_api]
impl INode for CarRightState {
    
    fn ready(&mut self) {
        self.name = "CarRightState".to_string();
        self.car_sprite_frames = SpriteAnimationLoader::load_sprite_frames(&self.path_sprite_frames.to_string());
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
        let parent =  match parent.get_parent() {
            Some(parent) => parent,
            None => panic!("pas de parent2"),
    };
 
        let mut animator: Gd<AnimatedSprite2D> = parent.get_node_as("AnimatedSprite2D");
        println!("aaaa {animator}");
        SpriteAnimationLoader::set_sprite_frames(&mut animator, &self.car_sprite_frames);
    }

    fn exit(&mut self) {
        
    }

    fn update(&mut self, delta: f64) {
        
    }

    fn physics_update(&mut self, delta: f64) {
        
    }

    fn connect(&mut self, singal_name: String, callable: Callable) {
        
    }

    }




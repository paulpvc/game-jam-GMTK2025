use godot::classes::{Area2D, Camera2D, CharacterBody2D, Node};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct CameraController {
    area: Option<Gd<Area2D>>,
    base: Base<Node>,
    direction: Vector2,
    target: Option<Gd<CharacterBody2D>>,
    witness: Option<Gd<Node2D>>,
    fixed_y: f32,
    parent: Option<Gd<Node2D>>,
    is_car_child: bool,
    is_in_area: bool,
    root: Option<Gd<Node>>,
    needs_reparent: Option<bool>,
}

use godot::classes::INode;

#[godot_api]
impl INode for CameraController {
    fn ready(&mut self) {
        let parent = match self.base().get_parent() {
            Some(parent) => parent,
            None => panic!("pas de parent"),
        };

        let mut area: Gd<Area2D> = parent.get_node_as("Area2D");

        let callable: Callable = self.base().callable(&StringName::from("on_body_entered"));
        let signal_name: StringName = "body_entered".into();

        area.connect(&signal_name, &callable);

        let callable: Callable = self.base().callable(&StringName::from("on_body_exited"));
        let signal_name: StringName = "body_exited".into();

        area.connect(&signal_name, &callable);

        self.area = Some(area);

        self.witness = Some(parent.get_node_as("Witness"));

        let parent_2d = parent.cast::<Node2D>();

        self.fixed_y = parent_2d.get_global_position().y;

        self.is_car_child = false;
        if let Some(root) = parent_2d.get_parent() {
            self.root = Some(root);
        }
        self.is_in_area = false;

        self.needs_reparent = None;
    }

    fn physics_process(&mut self, _delta: f64) {
        let target = match &self.target {
            Some(target) => target.clone(), // Clone seulement la référence Gd
            None => return,
        };

        if !self.is_in_area {
            return;
        }

        let parent = match self.base().get_parent() {
            Some(parent) => parent,
            None => {
                godot_error!("CameraController: pas de parent dans physics_process");
                return;
            }
        };

        let mut parent_node2d: Gd<Node2D> = parent.cast::<Node2D>();
        let camera_pos = match &self.witness {
            Some(witness) => witness.get_global_position(),
            None => {
                godot_error!("CameraController: pas de witness pour la camera");
                return;
            }
        };

        let parent_position = parent_node2d.get_global_position();
        let velocity = target.get_velocity();

        // Déterminer si on doit changer de parent
        if velocity.x > 0.0 && !self.is_car_child {
            self.needs_reparent = Some(true);
        } else if velocity.x <= 0.0 && self.is_car_child {
            self.needs_reparent = Some(false);
        }

        // Toujours mettre à jour la position
        let position = Vector2::new(parent_position.x, self.fixed_y);
        parent_node2d.set_global_position(position);
    }

    fn process(&mut self, _delta: f64) {
        if let Some(should_attach) = self.needs_reparent.take() {
            self.handle_reparenting(should_attach);
        }
    }
}

#[godot_api]
impl CameraController {
    #[func]
    fn set_direction_from_state(&mut self, direction: Vector2) {
        self.direction = direction;
    }

    #[func]
    fn on_body_entered(&mut self, body: Gd<Node2D>) {
        // Approche minimale - juste stocker les flags
        godot_print!("Body entered: {}", body);
        if let Ok(character_body) = body.try_cast::<CharacterBody2D>() {
            self.target = Some(character_body);
        }
        self.is_in_area = true;
    }

    #[func]
    fn on_body_exited(&mut self, _body: Gd<Node2D>) {
        // Approche minimale - juste changer les flags
        self.is_in_area = false;
        if self.is_car_child {
            self.needs_reparent = Some(false);
        }
    }

    #[func]
    fn set_target(&mut self, target: Gd<Node2D>) {
        godot_print!("{target}");
        self.target = Some(target.cast::<CharacterBody2D>());
    }

    fn handle_reparenting(&mut self, should_attach_to_car: bool) {
        let mut target = match &mut self.target {
            Some(target) => target.clone(),
            None => return,
        };

        let mut parent = match &mut self.base().get_parent() {
            Some(parent) => parent.clone().cast::<Node2D>(),
            None => return,
        };
        let glob_pos = parent.get_global_position();
        if should_attach_to_car && !self.is_car_child {

            
            // Détacher du parent actuel
            if let Some(mut current_parent) = parent.get_parent() {
                current_parent.remove_child(&parent);
            }


            // Attacher à la voiture
            target.add_child(&parent);
                        self.is_car_child = true;
        } else if !should_attach_to_car && self.is_car_child {
            // Détacher de la voiture
            target.remove_child(&parent);

            // Rattacher au root
            if let Some(mut root) = self.root.clone() {
                root.add_child(&parent);
            }
            self.is_car_child = false;
        }
        parent.set_global_position(glob_pos);
    }
}

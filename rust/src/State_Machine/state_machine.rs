use std::collections::HashMap;

use godot::classes::Node;
use godot::prelude::*;

use crate::State_Machine::state::{StatesType, StateLogic, StateRegistry};


#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct StateMachine {
    #[export]
    init_state_name: GString,

    current_state: Option<StatesType>,
    states: HashMap<String, StatesType>,
    base: Base<Node>,
}

use godot::classes::INode;

#[godot_api]
impl INode for StateMachine {
    fn ready(&mut self) {

        

        for child in self.base().get_children().iter_shared() {
            if let Some(mut state) = StateRegistry::resolve(&child.clone()) {
                let name = state.name();
                godot_print!("{name}");
                let signal_name = "transitionned".to_string();
                state.connect(
                    &signal_name,
                    self.base().callable("on_child_transition"),
                );

                self.states.insert(name, state);
            }
        }

        self.current_state = match self.states.get(&self.init_state_name.to_string()) {
            Some(state) => {
                let mut state = state.clone();
                let name = state.name();
                println!("bbbb {name}");
                state.enter();
                Some(state)},
            None => panic!("Impossible d'initialiser l'état courant"),
        };
    }

    fn process(&mut self, delta: f64) {
        match &mut self.current_state {
            Some(state) => state.update(delta),
            _ => {}
        }
    }

    fn physics_process(&mut self, delta: f64) {
        match &mut self.current_state {
            Some(state) => state.physics_update(delta),
            _ => {}
        }
    }
}

#[godot_api]
impl StateMachine {

    #[func] 
    fn on_child_transition(&mut self, state_name: String, new_state_name: String) {
        godot_print!("changement appelé0");
        let current_state: &mut StatesType = match &mut self.current_state {
            Some(state) => state,
            None => panic!("on a perdu le current_state"),
        };

        if current_state.name() == state_name {
            let new_state = self.states.get(&new_state_name);
            let mut new_state_ex = match new_state {
                Some(new_state) => new_state.clone(),
                _ => panic!("l'état {new_state_name} n'existe pas"),
            };

            current_state.exit();
            new_state_ex.enter();
            self.current_state = Some(new_state_ex);
        }
        
    }
    
}

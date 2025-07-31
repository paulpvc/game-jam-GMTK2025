use crate::State_Machine::car::car_right::CarRightState;
use godot::classes::Node;
use godot::prelude::*;

pub trait StateLogic {
    // Méthode abstraite - chaque sous-type DOIT l'implémenter avec son nom unique
    fn name(&self) -> String;

    // Méthodes abstraites - doivent être implémentées par chaque sous-type
    fn enter(&mut self);
    fn exit(&mut self);
    fn update(&mut self, delta: f64);
    fn physics_update(&mut self, delta: f64);
    fn connect(&mut self, singal_name: String, callable: Callable);
}

#[derive(Clone)]
pub enum StatesType {
    CarRight(Gd<CarRightState>),
}

impl StateLogic for StatesType {
    fn name(&self) -> String {
        match self {
            StatesType::CarRight(state) => state.bind().name(),
        }
    }

    fn enter(&mut self) {
        match self {
            StatesType::CarRight(state) => state.bind_mut().enter(),
        }
    }

    fn exit(&mut self) {}

    fn update(&mut self, delta: f64) {
        match self {
            StatesType::CarRight(state) => state.bind_mut().update(delta),
        }
    }

    fn physics_update(&mut self, delta: f64) {}

    fn connect(&mut self, signal_name: String, callable: Callable) {
        match self {
            StatesType::CarRight(state) => state.bind_mut().connect(signal_name, callable),
        }
    }
}

pub struct StateRegistry;

impl StateRegistry {
    pub fn resolve(node: &Gd<Node>) -> Option<StatesType> {
        if let Ok(s) = node.clone().try_cast::<CarRightState>() {
            Some(StatesType::CarRight(s))
        }
        // else if let Some(s) = node.clone().try_cast::<CarLeftState>() {
        // Some(BoxedState::CarLeft(s))
        //}
        else {
            None
        }
    }
}

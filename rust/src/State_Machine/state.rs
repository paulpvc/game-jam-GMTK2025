use crate::State_Machine::car::car_down::CarDownState;
use crate::State_Machine::car::car_down_right::CarDownRightState;
use crate::State_Machine::car::car_right::CarRightState;
use crate::State_Machine::car::car_up_right::CarUpRightState;
use godot::classes::Node;
use godot::prelude::*;

use super::car::car_down_left::CarDownLeftState;
use super::car::car_left::CarLeftState;
use super::car::car_up::CarUpState;
use super::car::car_up_left::CarUpLeftState;

pub trait StateLogic {
    // Méthode abstraite - chaque sous-type DOIT l'implémenter avec son nom unique
    fn name(&self) -> String;

    // Méthodes abstraites - doivent être implémentées par chaque sous-type
    fn enter(&mut self);
    fn exit(&mut self);
    fn update(&mut self, delta: f64);
    fn physics_update(&mut self, delta: f64);
    fn connect(&mut self, singal_name: &String, callable: Callable);
}

#[derive(Clone)]
pub enum StatesType {
    CarRight(Gd<CarRightState>),
    CarUpRight(Gd<CarUpRightState>),
    CarDownRight(Gd<CarDownRightState>),
    CarDown(Gd<CarDownState>),
    CarDownLeft(Gd<CarDownLeftState>),
    CarLeft(Gd<CarLeftState>),
    CarUpLeft(Gd<CarUpLeftState>),
    CarUp(Gd<CarUpState>,)
}

impl StateLogic for StatesType {
    fn name(&self) -> String {
        match self {
            StatesType::CarRight(state) => state.bind().name(),
            StatesType::CarUpRight(state) => state.bind().name(),
            StatesType::CarDownRight(state) => state.bind().name(),
            StatesType::CarDown(state) => state.bind().name(),
            StatesType::CarDownLeft(state) => state.bind().name(),
            StatesType::CarLeft(state) => state.bind().name(),
            StatesType::CarUpLeft(state) => state.bind().name(),
            StatesType::CarUp(state) => state.bind().name(),
        }
    }

    fn enter(&mut self) {
        match self {
            StatesType::CarRight(state) => state.bind_mut().enter(),
            StatesType::CarUpRight(state) => state.bind_mut().enter(),
            StatesType::CarDownRight(state) => state.bind_mut().enter(),
            StatesType::CarDown(state) => state.bind_mut().enter(),
            StatesType::CarDownLeft(state) => state.bind_mut().enter(),
            StatesType::CarLeft(state) => state.bind_mut().enter(),
            StatesType::CarUpLeft(state) => state.bind_mut().enter(),
            StatesType::CarUp(state) => state.bind_mut().enter(),
        }
    }

    fn exit(&mut self) {}

    fn update(&mut self, delta: f64) {
        match self {
            StatesType::CarRight(state) => state.bind_mut().update(delta),
            StatesType::CarUpRight(state) => state.bind_mut().update(delta),
            StatesType::CarDownRight(state) => state.bind_mut().update(delta),
            StatesType::CarDown(state) => state.bind_mut().update(delta),
            StatesType::CarDownLeft(state) => state.bind_mut().update(delta),
            StatesType::CarLeft(state) => state.bind_mut().update(delta),
            StatesType::CarUpLeft(state) => state.bind_mut().update(delta),
            StatesType::CarUp(state) => state.bind_mut().update(delta),
        }
    }

    fn physics_update(&mut self, delta: f64) {
        match self {
            StatesType::CarRight(state) => state.bind_mut().physics_update(delta),
            StatesType::CarUpRight(state) => state.bind_mut().physics_update(delta),
            StatesType::CarDownRight(state) => state.bind_mut().physics_update(delta),
            StatesType::CarDown(state) => state.bind_mut().physics_update(delta),
            StatesType::CarDownLeft(state) => state.bind_mut().physics_update(delta),
            StatesType::CarLeft(state) => state.bind_mut().physics_update(delta),
            StatesType::CarUpLeft(state) => state.bind_mut().physics_update(delta),
            StatesType::CarUp(state) => state.bind_mut().physics_update(delta),
        }
    }

    fn connect(&mut self, signal_name: &String, callable: Callable) {
        match self {
            StatesType::CarRight(state) => state.bind_mut().connect(signal_name, callable),
            StatesType::CarUpRight(state) => state.bind_mut().connect(signal_name, callable),
            StatesType::CarDownRight(state) => state.bind_mut().connect(signal_name, callable),
            StatesType::CarDown(state) => state.bind_mut().connect(signal_name, callable),
            StatesType::CarDownLeft(state) => state.bind_mut().connect(signal_name, callable),
            StatesType::CarLeft(state) => state.bind_mut().connect(signal_name, callable),
            StatesType::CarUpLeft(state) => state.bind_mut().connect(signal_name, callable),
            StatesType::CarUp(state) => state.bind_mut().connect(signal_name, callable),
        }
    }
}

pub struct StateRegistry;

impl StateRegistry {
    pub fn resolve(node: &Gd<Node>) -> Option<StatesType> {
        if let Ok(s) = node.clone().try_cast::<CarRightState>() {
            Some(StatesType::CarRight(s))
        }
        else if let Ok(s) = node.clone().try_cast::<CarUpRightState>() {
         Some(StatesType::CarUpRight(s))
        }
        else if let Ok(s) = node.clone().try_cast::<CarDownRightState>() {
         Some(StatesType::CarDownRight(s))
        }
        else if let Ok(s) = node.clone().try_cast::<CarDownState>() {
         Some(StatesType::CarDown(s))
        }
        else if let Ok(s) = node.clone().try_cast::<CarDownLeftState>() {
         Some(StatesType::CarDownLeft(s))
        }
else if let Ok(s) = node.clone().try_cast::<CarLeftState>() {
         Some(StatesType::CarLeft(s))
        }
else if let Ok(s) = node.clone().try_cast::<CarUpLeftState>() {
         Some(StatesType::CarUpLeft(s))
        }
else if let Ok(s) = node.clone().try_cast::<CarUpState>() {
         Some(StatesType::CarUp(s))
        }

        else {
            None
        }
    }
}

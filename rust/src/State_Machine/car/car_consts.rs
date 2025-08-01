use godot::builtin::Vector2;

use godot::classes::Input;
use godot::global::godot_print;

pub const MAX_SPEED: f64 = 200.0;
pub const ACCELERATION: f64 = 50.0;
pub const DECELERATION: f64 = 100.0;
pub const TURN_THRESHOLD: f64 = 20.0;

use std::f32::consts::PI;

pub fn get_state_from_direction(velocity: Vector2) -> String {
    // Calculer l'angle en radians (-PI à PI)
    let angle = velocity.y.atan2(velocity.x);

    // Convertir en degrés et normaliser entre 0 et 360
    let mut degrees = angle * 180.0 / PI;
    if degrees < 0.0 {
        degrees += 360.0;
    }

    //godot_print!("{degrees}");

    // Déterminer l'état basé sur l'angle
    // Chaque direction couvre 45 degrés (360/8 directions)
    match degrees {
        // Droite (337.5° à 22.5°)
        d if d >= 337.5 || d < 22.5 => "CarRightState".to_string(),

        // Haut-Droite (22.5° à 67.5°)
        d if d >= 22.5 && d < 67.5 => "CarDownRightState".to_string(),

        // Haut (67.5° à 112.5°)
        d if d >= 67.5 && d < 112.5 => "CarDownState".to_string(),

        // Haut-Gauche (112.5° à 157.5°)
        d if d >= 112.5 && d < 157.5 => "CarDownLeftState".to_string(),

        // Gauche (157.5° à 202.5°)
        d if d >= 157.5 && d < 202.5 => "CarLeftState".to_string(),

        // Bas-Gauche (202.5° à 247.5°)
        d if d >= 202.5 && d < 247.5 => "CarUpLeftState".to_string(),

        // Bas (247.5° à 292.5°)
        d if d >= 247.5 && d < 292.5 => "CarUpState".to_string(),

        // Bas-Droite (292.5° à 337.5°)
        d if d >= 292.5 && d < 337.5 => "CarUpRightState".to_string(),

        _ => String::new(),
    }
}

pub fn update_velocity(facing_direction: Vector2, mut velocity: Vector2, delta: &f64) -> Vector2 {
    let direction: f32 = Input::singleton().get_axis("ui_left", "ui_right");
    let direction_rotation: f32 = Input::singleton().get_axis("ui_up", "ui_down");

    let rotation_vector = -facing_direction.orthogonal();

    if direction != 0.0 {
        let target_speed = direction * MAX_SPEED as f32;
        let forward = facing_direction.normalized();

        let toward_vector = (forward + rotation_vector * direction_rotation) * target_speed;
        //godot_print!("direction_rotation: {direction_rotation}, toward vector: {toward_vector}, facing direction: {facing_direction}");
        velocity = velocity.move_toward(toward_vector, (ACCELERATION * delta) as f32);
        velocity
    } else {
        velocity = velocity.move_toward(Vector2::ZERO, (DECELERATION * delta) as f32);
        velocity
    }
}

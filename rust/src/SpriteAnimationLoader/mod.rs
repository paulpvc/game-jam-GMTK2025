use godot::prelude::*;
use godot::builtin::GString;
use godot::classes::{ResourceLoader, AnimatedSprite2D,SpriteFrames};

pub fn load_sprite_frames(path: &str) -> Gd<SpriteFrames> {
    let mut loader = ResourceLoader::singleton();

    let path = GString::from(path);

    // Charger la ressource
    if let Some(resource) = loader.load(&path.clone()) {
        // Tenter de caster vers SpriteFrames
        if let Ok(sprite_frames) = resource.try_cast::<SpriteFrames>() {
            godot_print!("Animation chargée depuis {}", path);
            sprite_frames
        } else {
           panic!("La ressource chargée n'est pas un SpriteFrames");
        }
    } else {
         panic!("Impossible de charger la ressource : {}", path);
    }
}

pub fn set_sprite_frames (animated_sprite: &mut Gd<AnimatedSprite2D>, sprite_frames: &Gd<SpriteFrames>) {
    animated_sprite.set_sprite_frames(sprite_frames);
}

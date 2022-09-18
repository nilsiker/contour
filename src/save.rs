use bevy::prelude::*;

pub struct Save {
    data: i32,
}

pub fn save_exists(save_res: Option<Res<Save>>) -> bool {
    match save_res {
        Some(_) => true,
        None => false,
    }
}

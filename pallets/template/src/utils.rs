use frame_support::runtime_print;

use crate::Coord;

pub fn get_hash_u32<T: frame_system::Config>() -> u32 {
    let hash = <frame_system::Pallet<T>>::parent_hash();
    let bytes = hash.as_ref();
    u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

fn get_random(seed: u32, skip: u32, max: u32) -> u32 {
    let skip = if skip == 0 { 1 } else { skip };
    let new_seed = seed / skip;
    new_seed % max
}

pub fn get_random_x<T: frame_system::Config>(max: u32, index: u32) -> u32 {
    let hash = get_hash_u32::<T>();
    let result = get_random(hash, index, max);
    runtime_print!("[utils] x: {:?}", result);
    result
}

pub fn get_random_y<T: frame_system::Config>(max: u32, index: u32) -> u32 {
    let hash = get_hash_u32::<T>();
    let result = get_random(hash, 100 + index, max);
    runtime_print!("[utils] y: {:?}", result);
    result
}

pub fn get_distance(coord1: Coord, coord2: Coord) -> u32 {
    let dx = (coord1.x as i32 - coord2.x as i32).abs() as u32;
    let dy = (coord1.y as i32 - coord2.y as i32).abs() as u32;
    dx + dy
}



use crate::addresses::{BASE_ADDRESS, GAME_DATA, LEVEL, CURRENT_SUN, GAME_PAUSED, GAME_STATE};
use crate::mem_utils::{read_value_from_offsets, write_value_to_offsets};

const SUN_ADDRESS: &[u32] = &[GAME_DATA, CURRENT_SUN];
const LEVEL_ADDRESS: &[u32] = &[GAME_DATA, LEVEL];
const PAUSE_ADDRESS: &[u32] = &[GAME_DATA, GAME_PAUSED];

fn print_error(action: &str, e: &windows::core::Error) {
    println!("Failed to {}. Error: {:?}", action, e);
}
#[allow(dead_code)]
pub fn get_current_level(process_id: u32) {
    match read_value_from_offsets(process_id, BASE_ADDRESS, LEVEL_ADDRESS) {
        Ok(value) => println!("Retrieved Current Level: {}", value),
        Err(e) => print_error("retrieve the current level", &e),
    }
}
#[allow(dead_code)]
pub fn set_sun_value(process_id: u32, set_value_amount: u32) {
    match write_value_to_offsets(process_id, BASE_ADDRESS, SUN_ADDRESS, set_value_amount) {
        Ok(_) => println!("Successfully wrote new sun value: {}", set_value_amount),
        Err(e) => print_error("write the new sun value", &e),
    }
}

pub fn change_sun_value(process_id: u32, change_value_amount: i32) {
    match get_game_state(process_id) {
        Ok(3) => println!("Player is currently in game"),
        Ok(_) => return,
        Err(e) => print_error("Get the game state", &e),
    }
    match is_game_paused(process_id) {
        Ok(0) => println!("Game is not paused"),
        Ok(_) => return,
        Err(e) => print_error("Get game paused", &e),
    }
    match current_sun_value(process_id) {
        Ok(value) => {
            let new_value = (value as i32 + change_value_amount).max(0) as u32;

            match write_value_to_offsets(process_id, BASE_ADDRESS, SUN_ADDRESS, new_value) {
                Ok(_) => println!("Successfully wrote new sun value: {}", new_value),
                Err(e) => print_error("write the new sun value", &e),
            }
        },
        Err(e) => print_error("retrieve the sun value", &e),
    }
}

pub fn is_game_paused(process_id: u32) -> Result<u32, windows::core::Error> {
    read_value_from_offsets(process_id, BASE_ADDRESS, PAUSE_ADDRESS)
}


pub fn get_game_state(process_id: u32) -> Result<u32, windows::core::Error> {
    read_value_from_offsets(process_id, BASE_ADDRESS, &[GAME_STATE])
}

fn current_sun_value(process_id: u32) -> Result<u32, windows::core::Error> {
    read_value_from_offsets(process_id, BASE_ADDRESS, SUN_ADDRESS)
}
use crate::addresses::{BASE_ADDRESS, GAME_DATA, LEVEL, CURRENT_SUN};
use crate::mem_utils::{read_value_from_offsets, write_value_to_offsets};

const SUN_ADDRESS: &[u32] = &[GAME_DATA, CURRENT_SUN];
const LEVEL_ADDRESS: &[u32] = &[GAME_DATA, LEVEL];

fn print_error(action: &str, e: &windows::core::Error) {
    println!("Failed to {}. Error: {:?}", action, e);
}

pub fn get_current_level(process_id: u32) {
    match read_value_from_offsets(process_id, BASE_ADDRESS, LEVEL_ADDRESS) {
        Ok(value) => println!("Retrieved Current Level: {}", value),
        Err(e) => print_error("retrieve the current level", &e),
    }
}
// TODO: Check if attempting to set value to >0 and set to 0
pub fn set_sun_value(process_id: u32, set_value_amount: u32) {
    match write_value_to_offsets(process_id, BASE_ADDRESS, SUN_ADDRESS, set_value_amount) {
        Ok(_) => println!("Successfully wrote new sun value: {}", set_value_amount),
        Err(e) => print_error("write the new sun value", &e),
    }
}

pub fn change_sun_value(process_id: u32, change_value_amount: i32) {
    match current_sun_value(process_id) {
        Ok(value) => {
            // Ensure value does not go below 0
            let new_value = (value as i32 + change_value_amount).max(0) as u32;

            match write_value_to_offsets(process_id, BASE_ADDRESS, SUN_ADDRESS, new_value) {
                Ok(_) => println!("Successfully wrote new sun value: {}", new_value),
                Err(e) => print_error("write the new sun value", &e),
            }
        },
        Err(e) => print_error("retrieve the sun value", &e),
    }
}

fn current_sun_value(process_id: u32) -> Result<u32, windows::core::Error> {
    read_value_from_offsets(process_id, BASE_ADDRESS, SUN_ADDRESS)
}
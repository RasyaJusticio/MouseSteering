#![windows_subsystem = "windows"]

mod config;
mod emulator;
mod gui;
mod key;
mod processor;

use crate::gui::Visualizer;
use config::Config;
use device_query::{DeviceQuery, DeviceState};
use emulator::Emulator;
use key::Key;
use processor::Processor;
use std::{thread, time::Duration};

fn main() {
    let mut emulator = Emulator::new();
    let mut visualizer = Visualizer::new();

    let device_state = DeviceState::new();

    let config = Config::new();
    let steering_config = &config.steering_config;
    let throttle_config = &config.throttle_config;

    let mut steer_key = Key::new();
    let mut throttle_key = Key::new();

    loop {
        let mouse_position = device_state.get_mouse().coords;
        let pressed_keys = device_state.get_keys();

        if pressed_keys.contains(&config.steering_config.toggle_key) {
            steer_key.update(true);
        } else {
            steer_key.update(false);
        }

        if pressed_keys.contains(&config.throttle_config.toggle_key) {
            throttle_key.update(true);
        } else {
            throttle_key.update(false);
        }

        if steering_config.enabled && !steer_key.value {
            visualizer.hor_lock = false;

            let normal_value = Processor::normalize(mouse_position.0, 0, config.screen_width);

            let horizontal_value = Processor::process(normal_value, &steering_config);

            emulator.gamepad.thumb_lx = Processor::to_thumb_val(horizontal_value);

            let (lval, rval) = if horizontal_value <= 0.0 {
                (horizontal_value.abs(), 0.0)
            } else {
                (0.0, horizontal_value.abs())
            };
            visualizer.lval = lval;
            visualizer.rval = rval;
        } else {
            emulator.gamepad.thumb_lx = Processor::to_thumb_val(0.0);

            visualizer.hor_lock = true;
            // visualizer.lval = 0.0;
            // visualizer.rval = 0.0;
        }

        if throttle_config.enabled && !throttle_key.value {
            visualizer.ver_lock = false;

            let normal_value = Processor::normalize(mouse_position.1, 0, config.screen_height);

            let vertical_value = Processor::process(normal_value, &throttle_config);

            let (left_trigger, right_trigger) = if vertical_value <= 0.0 {
                (0.0, vertical_value.abs())
            } else {
                (vertical_value.abs(), 0.0)
            };

            emulator.gamepad.left_trigger = Processor::to_trigger_val(left_trigger);
            emulator.gamepad.right_trigger = Processor::to_trigger_val(right_trigger);

            visualizer.tval = right_trigger;
            visualizer.bval = left_trigger;
        } else {
            // emulator.gamepad.left_trigger = Processor::to_trigger_val(0.0);
            // emulator.gamepad.right_trigger = Processor::to_trigger_val(0.0);
            
            visualizer.ver_lock = true;
            // visualizer.tval = 0.0;
            // visualizer.bval = 0.0;
        }

        emulator.emulate();
        visualizer.draw();

        if visualizer.should_close() {
            break;
        }

        thread::sleep(Duration::from_millis(1));
    }
}

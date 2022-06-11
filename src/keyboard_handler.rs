use device_query::{DeviceQuery, DeviceState, Keycode};

use crate::map::Map;

pub struct KeyboardHandler { // KEYBOARD GO AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
    device_state: DeviceState,
    keys: Vec<Keycode>,
}

impl KeyboardHandler {
    pub fn new() -> Self {
        Self {
            device_state: DeviceState::new(),
            keys: vec!(),
        }
    }

    pub fn key_loop(&mut self, mut map: Map) -> Map { // This should be called from a different loop, than the render loop
        self.keys = self.device_state.get_keys();
        for key in &self.keys {
            match key {
                Keycode::W | Keycode::A | Keycode::S | Keycode::D => map.move_player(key),
                _ => () // Do nothing in any other cases
            }
        }
        map
    }

    pub fn is_key_pressed(&self, key: Keycode) -> bool {
        self.keys.contains(&key)
    }

    // DEBUG

    pub fn print_pressed_keys(&mut self) {
        loop {
            self.keys = self.device_state.get_keys();
            if self.keys.len() > 0 {
                for key in self.keys.iter() {
                    println!("Pressed key: {:?}", key);
                }
                println!("---------------------------");
            }
        }
    }
}
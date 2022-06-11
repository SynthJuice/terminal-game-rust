use device_query::Keycode;

mod keyboard_handler;
mod map;
mod renderer;
pub use crate::keyboard_handler::KeyboardHandler;
pub use crate::map::Map;
pub use crate::renderer::Renderer;

fn main() {
    let mut handler: KeyboardHandler = KeyboardHandler::new();
    
    let width: u32 = 64;
    let height: u32 = 16;
    let mut map = Map::new(width, height);
    let renderer = Renderer::new(width, height);
    
    map.fill_map();
    
    'game: loop {
        map = handler.key_loop(map);
        
        if handler.is_key_pressed(Keycode::Escape) {
            println!("EXIT");
            break 'game;
        }
        
        map.map_loop();
        
        renderer.clear_console();
        renderer.print_map(&map.character_map);
        
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 144));
    }
}

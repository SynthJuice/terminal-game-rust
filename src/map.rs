use device_query::Keycode;

pub struct Map {
    pub width: u32,
    pub height: u32,
    pub character_map: Vec<Vec<char>>,
    pub player_x: u32,
    pub player_y: u32,
    pub player_char: char
}

impl Map {
    pub fn new(map_width: u32, map_height: u32) -> Self {
        Self {
            width: map_width, 
            height: map_height, 
            character_map: vec!(vec!(' ' as char; map_height as usize); map_width as usize),
            player_x: 10,
            player_y: 10,
            player_char: '🡐'
        }
    }

    pub fn fill_map(&mut self) {
        for x in 0 .. self.width {
            for y in 0 .. self.height {
                let mut ch: char = ' ';
                if x == 0|| y == 0 || x == self.width - 1 || y == self.height - 1 {
                    ch = '#';
                }
                self.character_map[x as usize][y as usize] = ch;
            }
        }

        self.character_map[self.player_x as usize][self.player_y as usize] = self.player_char;
    }

    pub fn map_loop(&mut self) {
        self.character_map[self.player_x as usize][self.player_y as usize] = self.player_char;
    }

    pub fn get_char(&self, x: u32, y: u32) -> char {
        self.character_map[x as usize][y as usize]
    }

    pub fn set_char(&mut self, x: u32, y: u32, character: char) {
        self.character_map[x as usize][y as usize] = character;
    }

    pub fn move_player(&mut self, dir: &Keycode) {
        self.character_map[self.player_x as usize][self.player_y as usize] = ' ';
        if dir == &Keycode::W {
            self.player_y -= 1;
            self.player_char = '↑';
        } else if dir == &Keycode::A {
            self.player_x -= 1;
            self.player_char = '🡐';
        } else if dir == &Keycode::S {
            self.player_y += 1;
            self.player_char = '🡓';
        } else if dir == &Keycode::D {
            self.player_x += 1;
            self.player_char = '🡒';
        }

        if self.player_x > self.width - 1 {
            self.player_x = 0;
        }

        if self.player_y > self.height - 1 {
            self.player_y = 0;
        }
    }
}
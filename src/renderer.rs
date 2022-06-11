pub struct Renderer {
    render_buffer: Vec<Vec<char>>,
    width: u32,
    height: u32
}

impl Renderer {

    pub fn new(rb_width: u32, rb_height: u32) -> Self {
        Self { 
            render_buffer: vec!(vec!(0 as char; rb_width as usize); rb_height as usize),
            width: rb_width,
            height: rb_height
        }
    }

    pub fn clear_console(&self) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

    pub fn print_map(&self, map: &Vec<Vec<char>>) {
        for y in 0 .. self.height {
            for x in 0 .. self.width {
                if map[x as usize][y as usize] == '#' {
                    print!("\x1b[45m");
                    print!("{}", map[x as usize][y as usize]);
                    print!("\x1b[0m");
                    continue;
                }
                print!("{}", map[x as usize][y as usize]);
            }
            println!();
        }
    }

    pub fn print_buffer(&self) {
        for y in 0 .. self.height {
            for x in 0 .. self.width {
                if self.render_buffer[x as usize][y as usize] == '#' {
                    print!("\x1b[45m");
                    print!("{}", self.render_buffer[x as usize][y as usize]);
                    print!("\x1b[0m");
                    continue;
                }
                print!("{}", self.render_buffer[x as usize][y as usize]);
            }
            println!();
        }
    }
}
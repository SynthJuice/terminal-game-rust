pub struct Entity {
    let mut x: i32;
    let mut y: i32;
}

impl Entity {
    pub fn new(mut entity_x: i32, mut entity_y: i32) -> Self {
        Self { entity_x, entity_y }
    }

    pub fn move_entity(dir: Direction) { // This could be done here by passing along the character array to this method. It could also be done by letting the map do the moving of entities. ( DONT DO IT THROUGH THE MAP )

    }
}
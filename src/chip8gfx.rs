pub trait Interface {
    fn draw_sprite(&self, x:u8, y:u8, sprite:&[u8]) -> ();

    fn clear_screen(&self) -> ();
}

pub trait Init {
    fn init() -> Box<dyn Interface>;
}
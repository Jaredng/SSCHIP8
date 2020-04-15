use crate::chip8gfx;

pub struct builder {

}

impl chip8gfx::Init for builder {
    fn init() -> Box<dyn chip8gfx::Interface> {
        return Box::new(tgfx{});
    }
}

pub struct tgfx {

}

impl chip8gfx::Interface for tgfx {
    fn draw_sprite(&self, x:u8, y:u8, sprite:&[u8]){
        
    }

    fn clear_screen(&self) {
        
    }
}
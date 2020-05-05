pub trait Interface {
    //Draw a sprite at coordinate (x, y) from the supplied data.
    //Sprite width is 8 pixels.
    //Return 0x01 if any screen pixels are changed from on to off when the sprite is drawn
    //0x00 otherwise.
    fn draw_sprite(&mut self, x:u8, y:u8, sprite:&[u8]) -> u8;

    fn clear_screen(&mut self) -> ();
}
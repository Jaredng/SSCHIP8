pub trait Interface {

    //Returns a u16 bitfield indicating which keys are pressed.
    fn update(&self) -> u16;
}
pub trait Interface {

    //Return true if given key is down when function called. False otherwise.
    fn check_pressed(&self, key: u8) -> bool;

    //Wait for the next keypress, then return its key ID
    fn get_keypress(&self) -> u8;
}
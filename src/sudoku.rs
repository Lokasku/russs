pub struct Cell {
    value: Option<u32>,
    a: u8,
    b: u8
}
impl Cell {
    pub fn get_value(self) -> Option<u32> { self.value }
    pub fn get_a(&self) -> u8 { self.a }
    pub fn get_b(&self) -> u8 { self.b }
}
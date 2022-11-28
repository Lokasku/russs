pub struct Cell<'a> {
    value: &'a str,
    a: u8,
    b: u8
}
impl<'a> Cell<'a> {
    pub fn get_value(self) -> &'a str { self.value }
    pub fn get_a(&self) -> u8 { self.a }
    pub fn get_b(&self) -> u8 { self.b }
}
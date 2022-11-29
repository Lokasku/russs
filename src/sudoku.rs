#[derive(Debug)]
pub struct Cell {
    pub value: Option<u32>,
    pub a: u32,
    pub b: u32
}

impl Cell {
    pub fn get_value(self) -> Option<u32> { self.value }
    pub fn get_a(&self) -> u32 { self.a }
    pub fn get_b(&self) -> u32 { self.b }
}

#[derive(Debug)]
pub struct Grid {
    pub content: Vec<Vec<Option<u32>>>
}

impl Grid {
    pub fn  solve() {}

    pub fn set(&mut self, cell: Cell, v: u32) {
        if v > 9 {
            panic!("{:?} must be between 0 and 9.", v);}
        self.content[cell.get_a() as usize][cell.get_b() as usize] = Some(v)
    }

    pub fn unset(&mut self, cell: Cell) {
        self.content[cell.get_a() as usize][cell.get_b() as usize] = None
    }

    pub fn is_valid() {}
    pub fn is_full() {}

    pub fn first_empty_cell(&self) -> Result<Cell, ()> {
        for i_line in 0..9 {
            for i_num in 0..9 {
                match self.content[i_line][i_num] {
                    None => return Ok(Cell {
                        value: self.content[i_line][i_num],
                        a: i_line as u32,
                        b: i_num as u32
                    }),
                    _ => continue
                };
            }
        }
        Err(())
    }

    pub fn get_as() {}
    pub fn get_possible_digits() {}
}
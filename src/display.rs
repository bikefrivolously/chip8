struct Display {
    screen: [[bool; 32]; 64]
}

impl Display {
    pub fn new() -> Display {
        display {
            screen: [[false; 32]; 64]
        }
    }

    pub fn clear(&mut self) {
        for row in self.display {
            for col in row {
                self.display[row][col] = false;
            }
        }
    }
}

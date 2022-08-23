pub struct Anim {
    current: usize,
    start: usize,
    end: usize,
}

impl Anim {
    pub const fn new(start: usize, end: usize) -> Self {
        Anim {
            current: start,
            start,
            end,
        }
    }

    pub fn step(&mut self) -> usize {
        let temp = self.current;
        self.current += 1;
        if self.current > self.end {
            self.current = self.start;
        }

        temp
    }
}
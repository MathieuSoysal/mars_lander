
#[derive(Copy, Clone, Debug)]
pub struct IterLimit {
    max: u64,
    cur: u64,
}

impl IterLimit {
    pub fn new(max: u64) -> IterLimit {
        IterLimit { max, cur: 0 }
    }

    pub fn inc(&mut self) {
        self.cur += 1;
    }

    pub fn reached(&self) -> bool {
        self.cur >= self.max
    }

    pub fn reset(&mut self) {
        self.cur = 0;
    }

    pub fn get(&self) -> u64 {
        self.cur
    }
}

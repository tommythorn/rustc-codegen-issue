pub struct State {
    b: [u64; 64],
    c: u64,
    d: u64,
    e: u64,
}

impl State {
    pub fn compute(&mut self, n: usize) -> u64 {
        let mut f = 0;

        for _ in 0..n {
            let p = self.d.trailing_zeros() as usize;
            self.d &= self.d - 1;
            f |= 1 << p;
            self.c -= 1 << p;
            self.e += self.b[p];
        }

        f
    }
}

#[no_mangle]
pub extern "C" fn interesting(s: &mut State, n: usize) -> u64 {
    s.compute(n)
}

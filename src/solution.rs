pub struct Fibonacci {
    t1: u128,
    t2: u128,
}

impl Fibonacci {
    /// Create new `Fibonacci`.то
    pub fn new() -> Fibonacci {
        let fib = Fibonacci {
            t1: 0,
            t2: 0,
        };

        return fib;
    }

    /// Calculate the n-th Fibonacci number.
    ///
    /// This shall not change the state of the iterator.
    /// The calculations shall wrap around at the boundary of u8.
    /// The calculations might be slow (recursive calculations are acceptable).
    pub fn fibonacci(n: usize) -> u8 {
        if n > 13 || n == 0 { return 0; }
        if n == 1 { return 1; }
        return Self::fibonacci(n-1) + Self::fibonacci(n-2);
    }
}

impl Iterator for Fibonacci {
    type Item = u128;
    
    /// Calculate the next Fibonacci number.
    ///
    /// The first call to `next()` shall return the 0th Fibonacci number (i.e., `0`).
    /// The calculations shall not overflow and shall not wrap around. If the result
    /// doesn't fit u128, the sequence shall end (the iterator shall return `None`).
    /// The calculations shall be fast (recursive calculations are **un**acceptable).
    fn next(&mut self) -> Option<Self::Item> {
        if self.t1 == 0 && self.t2 == 0 {
            self.t1 = 1;
            return Some(0);
        }
        if self.t1 == 1 && self.t2 == 0 {
            self.t2 = 1;
            self.t1 = 0;
            return Some(1);
        }

        let res = std::panic::catch_unwind(|| {self.t1 + self.t2});
        if res.is_err() { return None; }
            
        let t3 = self.t1 + self.t2;
        self.t1 = self.t2;
        self.t2 = t3;
        
        return Some(self.t2);
    }
}

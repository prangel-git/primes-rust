use std::collections::HashMap;

type Container = HashMap<u64, u64>;

pub struct PrimeSieve {
    current: u64,
    to_cross: Container,
}

impl PrimeSieve {
    pub fn new() -> Self {
        PrimeSieve {
            current: 2u64,
            to_cross: HashMap::new(),
        }
    }
}

impl Iterator for PrimeSieve {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == 2 {
            self.current = 3;
            Some(2)
        } else {
            while let Some(&step) = self.to_cross.get(&self.current) {
                self.to_cross.remove(&self.current);

                let mut next = self.current + step;

                while self.to_cross.contains_key(&next) {
                    next += step;
                }

                self.to_cross.insert(next, step);
                self.current += 2;
            }

            let prime = self.current;
            let key = prime * prime;
            let value = 2 * prime;

            self.to_cross.insert(key, value);
            self.current += 2;

            Some(prime)
        }
    }
}

pub struct Perms {
    c: Vec<usize>,
    i: usize,
    first_iter: bool,
    len: usize,
}

impl Perms {
    pub fn new(len: usize) -> Self {
        Self {
            c: vec![0; len],
            i: 0,
            first_iter: true,
            len,
        }
    }
}

impl Iterator for Perms {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let mut pair = (0, 0);

        if self.first_iter {
            self.first_iter = false;
            self.i += 1;
            return Some(pair);
        }

        while self.i < self.len {
            if self.c[self.i] < self.i {
                if self.i % 2 == 0 {
                    pair = (0, self.i);
                } else {
                    pair = (self.c[self.i], self.i);
                }

                self.c[self.i] += 1;
                self.i = 0;

                return Some(pair);
            } else {
                self.c[self.i] = 0;
                self.i += 1;
            }
        }

        None
    }
}

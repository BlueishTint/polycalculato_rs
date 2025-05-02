pub struct Permutations<T: Copy, const N: usize> {
    arr: [T; N],
    c: [usize; N],
    i: usize,
    first_iter: bool,
}

impl<T: Copy, const N: usize> Permutations<T, N> {
    pub fn new(arr: [T; N]) -> Self {
        Permutations {
            arr,
            c: [0; N],
            i: 0,
            first_iter: true,
        }
    }
}

impl<T: Copy, const N: usize> Iterator for Permutations<T, N> {
    type Item = [T; N];

    fn next(&mut self) -> Option<Self::Item> {
        if self.first_iter {
            self.first_iter = false;
            self.i += 1;
            return Some(self.arr);
        }

        while self.i < N {
            if self.c[self.i] < self.i {
                if self.i % 2 == 0 {
                    self.arr.swap(0, self.i);
                } else {
                    self.arr.swap(self.c[self.i], self.i);
                }

                self.c[self.i] += 1;
                self.i = 0;

                return Some(self.arr);
            } else {
                self.c[self.i] = 0;
                self.i += 1;
            }
        }

        None
    }
}

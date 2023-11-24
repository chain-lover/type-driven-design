use std::{thread::sleep, time::Duration};

const CLEAR: &str = "\x1B[2J\x1B[1;1H";

struct Progress<I> {
    iter: I,
    i: usize,
    bound: Option<usize>,
}

impl<I> Progress<I>
where
    I: ExactSizeIterator,
{
    pub fn with_bound(mut self) -> Self {
        self.bound = Some(self.iter.len());
        self
    }
}

impl<I> Progress<I> {
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            i: 0,
            bound: None,
        }
    }
}

impl<I> Iterator for Progress<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        print!("{}", CLEAR);
        match self.bound {
            Some(bound) => println!("[{}{}]", "*".repeat(self.i), " ".repeat(bound - self.i)),
            None => println!("{}", "*".repeat(self.i)),
        }
        self.i += 1;
        self.iter.next()
    }
}

trait ProgressTraitExt: Sized {
    fn progress(self) -> Progress<Self>;
}

impl<Iter> ProgressTraitExt for Iter
where
    Iter: Iterator,
{
    fn progress(self) -> Progress<Self> {
        Progress::new(self)
    }
}

fn expensive_calculation(_n: &i32) {
    sleep(Duration::from_secs(1));
}

fn main() {
    let v = vec![1, 2, 3];

    for n in v.iter().progress().with_bound() {
        expensive_calculation(n);
    }
}

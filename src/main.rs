use std::{thread::sleep, time::Duration};

const CLEAR: &str = "\x1B[2J\x1B[1;1H";

struct Unbounded;
struct Bounded {
    bound: usize,
    delims: (char, char),
}

struct Progress<I, B> {
    iter: I,
    i: usize,
    bound: B,
}

trait ProgressDisplay: Sized {
    fn display<I>(&self, progress: &Progress<I, Self>);
}

impl ProgressDisplay for Unbounded {
    fn display<I>(&self, progress: &Progress<I, Self>) {
        println!("{}", "*".repeat(progress.i));
    }
}

impl ProgressDisplay for Bounded {
    fn display<I>(&self, progress: &Progress<I, Self>) {
        println!(
            "{}{}{}{}",
            self.delims.0,
            "*".repeat(progress.i),
            " ".repeat(self.bound - progress.i),
            self.delims.1
        );
    }
}

impl<I> Progress<I, Unbounded> {
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            i: 0,
            bound: Unbounded,
        }
    }
}

impl<I> Progress<I, Unbounded>
where
    I: ExactSizeIterator,
{
    pub fn with_bound(self) -> Progress<I, Bounded> {
        let bound = Bounded {
            bound: self.iter.len(),
            delims: ('[', ']'),
        };
        Progress {
            iter: self.iter,
            i: self.i,
            bound,
        }
    }
}

impl<I> Progress<I, Bounded> {
    pub fn with_delims(mut self, delims: (char, char)) -> Self {
        self.bound.delims = delims;
        self
    }
}

impl<I, B> Iterator for Progress<I, B>
where
    I: Iterator,
    B: ProgressDisplay,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        print!("{}", CLEAR);
        self.bound.display(self);
        self.i += 1;
        self.iter.next()
    }
}

trait ProgressTraitExt: Sized {
    fn progress(self) -> Progress<Self, Unbounded>;
}

impl<Iter> ProgressTraitExt for Iter
where
    Iter: Iterator,
{
    fn progress(self) -> Progress<Self, Unbounded> {
        Progress::new(self)
    }
}

fn expensive_calculation(_n: &i32) {
    sleep(Duration::from_secs(1));
}

fn main() {
    let v = vec![1, 2, 3];

    for n in v.iter().progress().with_bound().with_delims(('<', '>')) {
        expensive_calculation(n);
    }
}

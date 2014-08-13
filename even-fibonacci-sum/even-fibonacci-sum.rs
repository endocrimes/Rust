use std::iter::AdditiveIterator;

use std::mem;
use std::num::One;

fn fibonacci<T: One>() -> Fibonacci<T> {
    fibonacci_with_init(One::one(), One::one())
}

fn fibonacci_with_init<T>(a0: T, a1: T) -> Fibonacci<T> {
    Fibonacci { current: a0, next: a1 }
}

struct Fibonacci<T> { current: T, next: T }

impl<T: Add<T,T>> Iterator<T> for Fibonacci<T> {
    fn next(&mut self) -> Option<T> {
        let new_next    = self.current + self.next;
        let new_current = mem::replace(&mut self.next, new_next);
        let retval = mem::replace(&mut self.current,   new_current);
        Some(retval)
    }
}

fn main() {
	let max = 4000000i;
	println!("Sequence result is: {}", sequencer(max));
}

fn sequencer(max: int) -> int {
	Fibonacci {current: 1i, next: 1i}
		.take_while(|&v| v < max)
		.filter(|&v| v % 2 == 0)
		.sum()
}
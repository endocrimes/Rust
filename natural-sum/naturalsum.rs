use std::iter::AdditiveIterator;

fn main() {
	let max = 1000i;
	println!("for in range_method is: {}", range_method(max));
	println!("recursive_method is: {}", recursive_method(0, max - 1));
	println!("filter_summation_method is: {}", filter_summation_method(max));
}

fn range_method(max: int) -> int {
	let mut accumulator = 0;
	let mut range = range(0i, max);
	for n in range {
		if n % 3 == 0 || n % 5 == 0 {
			accumulator += n;
		}
	}

	accumulator
}

fn recursive_method(accumulator: int, n: int) -> int {
	if n == 0 {
		accumulator
	}
	else if n % 3 == 0 || n % 5 == 0 {
		recursive_method(accumulator + n, n - 1)
	}
	else {
		recursive_method(accumulator, n - 1)
	}
}

fn filter_summation_method(max: int) -> int {
	range(0i, max)
		.filter(|&n| n % 3 == 0 || n % 5 == 0)
		.sum()
}
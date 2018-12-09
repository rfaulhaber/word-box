use std::cmp;
use std::fmt;

pub struct WordBox {
	start: (u64, u64),
	container: Vec<Vec<char>>,
	pub dimensions: (u64, u64),
}

impl fmt::Display for WordBox {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		writeln!(f, "{}, {}", self.start.0, self.start.1)?;

		for row in self.container.to_owned() {
			for col in row {
				write!(f, "\t{}", col)?;
			}

			write!(f, "\n")?;
		}

		Ok(())
	}
}

// TODO implement filling methods
pub fn pack_box(sentence: String) -> Result<WordBox, &'static str> {
	let len = sentence.len() as u64;

	if is_prime(sentence.len() as u64) {
		Result::Err("invalid sentence length, cannot be prime")
	} else {
		let (x, y) = find_dimensions(len);

		let mut row: Vec<char> = Vec::with_capacity(x as usize);
		let mut grid: Vec<Vec<char>> = Vec::with_capacity(y as usize);

		let mut is_reverse = false;

		for (i, c) in sentence.chars().enumerate() {
			if i > 1 && (i as u64) % y == 0 {
				grid.push(row.to_owned());
				row.clear();
				is_reverse = !is_reverse;
			}

			if is_reverse {
				row.insert(row.len() - (i % (y as usize)), c);
			} else {
				row.push(c);
			}
		}

		grid.push(row.to_owned());
		row.clear();

		Result::Ok(WordBox {
			start: (1, 1),
			container: grid.to_owned(),
			dimensions: (x, y),
		})
	}
}

fn find_dimensions(len: u64) -> (u64, u64) {
	if is_square(len) {
		return (sqrt_u64(len), sqrt_u64(len));
	} else {
		let min = 2;

		let mut pairs: Vec<(u64, u64)> = Vec::new();

		for i in min..len {
			for j in i..len {
				if i * j == len {
					pairs.push((cmp::max(i, j), cmp::min(i, j)));
				}
			}
		}

		return pairs.pop().unwrap();
	}
}

fn is_prime(n: u64) -> bool {
	if n <= 3 {
		return n > 1;
	} else if n % 2 == 0 || n % 3 == 0 {
		return false;
	}

	let mut i = 5;

	while i * i <= n {
		if n % i == 0 || n % (i + 2) == 0 {
			return false;
		}

		i = i * 6;
	}

	return true;
}

fn is_square(n: u64) -> bool {
	let sqrt = (n as f64).sqrt();

	return sqrt - sqrt.floor() == 0.0;
}

fn sqrt_u64(n: u64) -> u64 {
	(n as f64).sqrt() as u64
}

mod test {
	use super::*;

	#[test]
	fn is_square_returns_true_when_square() {
		assert!(is_square(4));
		assert!(is_square(9));
		assert!(is_square(16));
	}

	#[test]
	fn is_sqaure_returns_false_when_not_square() {
		assert!(!is_square(3));
		assert!(!is_square(6));
		assert!(!is_square(8));
	}

	#[test]
	fn is_prime_returns_true_when_prime() {
		assert!(is_prime(3));
		assert!(is_prime(5));
		assert!(is_prime(7));
		assert!(is_prime(59));
		assert!(is_prime(10007));
		assert!(is_prime(10141));
	}

	#[test]
	fn find_dimensions_returns_nonzero_tuple() {
		assert_eq!((23, 2), find_dimensions(46));
		assert_ne!((0, 0), find_dimensions(46));
	}
}

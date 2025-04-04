use super::{adrs, rw};

fn swap(arr: &mut Vec<u8>, a: &isize, b: &isize) {
	let tmp = arr[*a as usize];
	rw(&[adrs(a), adrs(arr)], &[adrs(&tmp)]);

	arr[*a as usize] = arr[*b as usize];
	rw(&[adrs(b), adrs(arr), adrs(a)], &[adrs(&arr)]);

	arr[*b as usize] = tmp;
	rw(&[adrs(&tmp)], &[adrs(&arr)]);
}

pub fn trace(arr: &mut Vec<u8>, low: &isize, high: &isize) {
	rw(&[adrs(low), adrs(high)], &[]);
	if low < high {
		let mut tmp = *high - *low;
		rw(&[adrs(high), adrs(low)], &[adrs(&tmp)]);
		tmp = tmp / 2;
		rw(&[adrs(&tmp)], &[adrs(&tmp)]);

		let mid = *low + tmp;
		rw(&[adrs(&low), adrs(&tmp)], &[adrs(&mid)]);
		let pivot = arr[mid as usize];
		rw(&[adrs(&mid), adrs(&arr)], &[adrs(&pivot)]);

		let mut i = *low;
		rw(&[adrs(&low)], &[adrs(&i)]);
		let mut j = *high;
		rw(&[adrs(&high)], &[adrs(&j)]);

		while i <= j {
			rw(&[adrs(&i), adrs(&j)], &[]);

			while arr[i as usize] < pivot {
				rw(&[adrs(&i), adrs(&arr), adrs(&pivot)], &[]);
				i = i + 1;
				rw(&[adrs(&i)], &[adrs(&i)]);
			}
			rw(&[adrs(&i), adrs(&arr), adrs(&pivot)], &[]);

			while arr[j as usize] > pivot {
				rw(&[adrs(&j), adrs(&arr), adrs(&pivot)], &[]);
				j = j - 1;
				rw(&[adrs(&j)], &[adrs(&j)]);
			}
			rw(&[adrs(&j), adrs(&arr), adrs(&pivot)], &[]);

			rw(&[adrs(&i), adrs(&j)], &[]);
			if i <= j {
				swap(arr, &i, &j);
				i = i + 1;
				rw(&[adrs(&i)], &[adrs(&i)]);
				j = j - 1;
				rw(&[adrs(&j)], &[adrs(&j)]);
			}
		}
		rw(&[adrs(&i), adrs(&j)], &[]);

		rw(&[adrs(&i), adrs(&high)], &[]);
		if i < *high {
			trace(arr, &i, high);
		}

		rw(&[adrs(&low), adrs(&j)], &[]);
		if *low < j {
			trace(arr, low, &j);
		}
	}
}

use tracer::quicksort::trace_quicksort;
mod state;
mod tracer;

fn best_case_scenario() {
	println!("Best Case Scenario");
	let mut arr = vec![
		1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
		26, 27, 28, 29, 30, 31, 32,
	];
	let high = (arr.len() - 1) as isize;
	trace_quicksort(&mut arr, &0, &high);
	tracer::reset_pc();
}

fn average_case_scenario() {
	println!("Average Case Scenario");
	let mut arr = vec![
		3, 14, 26, 17, 30, 25, 18, 28, 31, 27, 19, 11, 21, 24, 12, 8, 13, 22, 7, 1, 29, 10, 6, 23,
		2, 9, 5, 20, 16, 32, 15, 4,
	];
	let high = (arr.len() - 1) as isize;
	trace_quicksort(&mut arr, &0, &high);
	tracer::reset_pc();
}

fn worst_case_scenario() {
	println!("Worst Case Scenario");
	let mut arr = vec![
		32, 31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10,
		9, 8, 7, 6, 5, 4, 3, 2, 1,
	];
	let high = (arr.len() - 1) as isize;
	trace_quicksort(&mut arr, &0, &high);
	tracer::reset_pc();
}

fn main() {
	best_case_scenario();
	average_case_scenario();
	worst_case_scenario();
}

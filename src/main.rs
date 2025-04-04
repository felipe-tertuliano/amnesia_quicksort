use tracer::quicksort;
mod state;
mod tracer;

fn best_case_scenario() {
	println!("Best Case Scenario");
	let mut arr = vec![1, 2, 3, 4, 5, 6];
	let high = (arr.len() - 1) as isize;
	quicksort::trace(&mut arr, &0, &high);
	tracer::reset_pc();
}

fn average_case_scenario() {
	println!("Average Case Scenario");
	let mut arr = vec![6, 2, 4, 3, 5, 1];
	let high = (arr.len() - 1) as isize;
	quicksort::trace(&mut arr, &0, &high);
	tracer::reset_pc();
}

fn worst_case_scenario() {
	println!("Worst Case Scenario");
	let mut arr = vec![6, 5, 4, 3, 2, 1];
	let high = (arr.len() - 1) as isize;
	quicksort::trace(&mut arr, &0, &high);
	tracer::reset_pc();
}

fn main() {
	best_case_scenario();
	average_case_scenario();
	worst_case_scenario();
}

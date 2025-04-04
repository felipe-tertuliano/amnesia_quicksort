use crate::state;
use std::{collections::HashMap, fmt};
pub mod quicksort;

static PC: state::Type<u8> = state::initialize::<u8>();
static ADRS_HASH: state::Type<HashMap<usize, u8>> = state::initialize::<HashMap<usize, u8>>();

pub enum TraceTypeEnum {
	READ,
	WRITE,
}

impl fmt::Display for TraceTypeEnum {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let text = match self {
			TraceTypeEnum::READ => '0',
			TraceTypeEnum::WRITE => '1',
		};
		return write!(f, "{}", text);
	}
}

pub fn trace(t_type: TraceTypeEnum, value: usize) {
	let new_value: u8;
	let mut adrs_hash = state::get(&ADRS_HASH);

	if let Some(adrs) = adrs_hash.get(&value) {
		new_value = *adrs;
	} else {
		let pc = state::get(&PC);
		let tmp = pc + 1;
		adrs_hash.insert(value, tmp);
		state::set(&ADRS_HASH, adrs_hash);
		state::set(&PC, tmp);
		new_value = tmp;
	};

	println!("{t_type} {:x}", new_value);
}

pub fn adrs<T>(value: &T) -> usize {
	return value as *const T as usize;
}

pub fn rw(r: &[usize], w: &[usize]) {
	for e in r {
		trace(TraceTypeEnum::READ, *e);
	}
	for e in w {
		trace(TraceTypeEnum::WRITE, *e);
	}
}

/* pub fn get_pc() -> u8 {
	return state::get(&PC);
} */

pub fn reset_pc() {
	state::set(&PC, 0);
}

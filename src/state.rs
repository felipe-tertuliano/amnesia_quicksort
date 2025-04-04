use std::sync::{Mutex, OnceLock};

pub trait Value: Default + Clone {}
impl<T: Default + Clone> Value for T {}

pub struct Struct<T: Value> {
	value: T,
}

impl<T: Value> Struct<T> {
	fn new() -> Self {
		return Self {
			value: T::default(),
		};
	}
}

pub type Type<T> = OnceLock<Mutex<Struct<T>>>;

pub const fn initialize<T: Value>() -> Type<T> {
	return OnceLock::new();
}

pub fn get<T: Value>(from: &'static Type<T>) -> T {
	let mut value = T::default();
	if let Ok(mutex) = from.get_or_init(|| Mutex::new(Struct::new())).lock() {
		value = mutex.value.clone();
	}
	return value;
}

pub fn set<T: Value>(to: &'static Type<T>, value: T) {
	if let Ok(mut mutex) = to.get_or_init(|| Mutex::new(Struct::new())).lock() {
		mutex.value = value;
	}
}

extern crate osmpbfreader;

use osmpbfreader::borrowed_iter::borrowed_iter;

fn main() {
	let array = &[0];
	let value = Box::new(0);
	let mut value_ref = None;
	let iter = borrowed_iter(|v| {
		value_ref = Some(v);
		//~^ error
		array.iter()
	}, value);
	// Free
	drop(iter);
	// Read-after-free
	println!("{:?}", value_ref);
}

extern crate osmpbfreader;

use osmpbfreader::borrowed_iter::BorrowedIter;

fn main() {
	let array = &[0];
	let value = Box::new(0);
	let mut value_ref = None;
	let iter = BorrowedIter::new(value, |v| {
		value_ref = Some(v);
		//~^ error
		array.iter()
	});
	// Free
	drop(iter);
	// Read-after-free
	println!("{:?}", value_ref);
}

extern crate osmpbfreader;

use osmpbfreader::borrowed_iter::BorrowedIter;
static NONE: Option<i32> = None;

fn main() {
    let value = Box::new(0);
    let mut value_ref = None;
    let iter = BorrowedIter::new(value, |v| {
	//~^ error
	value_ref = Some(v);
	NONE.iter()
    });
    // Free
    drop(iter);
    // Read-after-free
    println!("{:?}", value_ref);
}

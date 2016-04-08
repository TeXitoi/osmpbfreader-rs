extern crate osmpbfreader;

use osmpbfreader::borrowed_iter::BorrowedIter;
use std::cell::RefCell;

static NONE: Option<i32> = None;

fn main() {
    let value = Box::new(0);
    let value_ref = RefCell::new(None);
    let iter = BorrowedIter::new(value, |v| {
	//~^ error
	*value_ref.borrow_mut() = Some(v);
	NONE.iter()
    });
    // Free
    drop(iter);
    // Read-after-free
    println!("{:?}", value_ref);
}

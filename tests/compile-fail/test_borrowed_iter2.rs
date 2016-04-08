// Copyright (c) 2014-2015 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

extern crate osmpbfreader;

use osmpbfreader::borrowed_iter;

#[inline(never)]
fn create() -> &'static i32 {
    let mut iter = {
        let v: Vec<i32> = vec![1, 2, 3];
        fn make_iter(v: &Vec<i32>) -> std::slice::Iter<i32> { v.iter() }
        Box::new(borrowed_iter::BorrowedIter::new(v, make_iter))
	//~^ error
    };
    iter.next().unwrap()
}

fn main() {
    let last = create();
    println!("{}", last);// use after free?
}

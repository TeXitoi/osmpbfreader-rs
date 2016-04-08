// Copyright (c) 2015 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

use objects::OsmObj;

pub struct BorrowedIter<T, I> {
    _borrow: Box<T>,
    iter: I,
}
impl<T, I> Iterator for BorrowedIter<T, I> where I: Iterator {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
impl<T, I> BorrowedIter<T, I> {
    pub fn new<'a>(t: T, f: fn(&'a T) -> I) -> BorrowedIter<T, I>
        where T: 'a, I: 'a + Iterator<Item = OsmObj>, I::Item: 'static
    {
        use std::mem;
        let b = Box::new(t);
        let i = f(unsafe { mem::transmute(&*b) });
        BorrowedIter { _borrow: b, iter: i }
    }
}

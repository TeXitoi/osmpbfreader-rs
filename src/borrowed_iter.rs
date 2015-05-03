// Copyright (c) 2014 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

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
pub fn borrowed_iter<'a, T: 'a, F, I>(f: F, t: T) -> BorrowedIter<T, I>
    where I: 'a + Iterator, I::Item: 'a, F: FnOnce(&'a T) -> I
{
    use std::mem;
    let b = Box::new(t);
    let i = f(unsafe { mem::transmute(&*b) });
    BorrowedIter { _borrow: b, iter: i }
}

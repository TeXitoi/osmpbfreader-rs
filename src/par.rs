// Copyright (c) 2014-2017 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

//! Parallel iterator for `OsmPbfReader`.

use std::collections::VecDeque;
use std::sync::Arc;
use futures::Future;
use futures_cpupool::{CpuPool, CpuFuture};

/// A parallel iterator over the `OsmObj` of an `OsmPbfReader`.
pub struct Iter<I: Iterator, U: IntoIterator, F> {
    pool: CpuPool,
    queue: VecDeque<CpuFuture<Vec<U::Item>, ()>>,
    iter: I,
    f: Arc<F>,
    cur_iter: ::std::vec::IntoIter<U::Item>,
}
impl<I: Iterator, U: IntoIterator, F> Iter<I, U, F>
    where F: Fn(I::Item) -> U,
          U::Item: Send + 'static,
          I::Item: Send + 'static,
          F: Sync + Send + 'static
{
    /// Creates a parallel iterator.
    pub fn new(iter: I, f: F) -> Iter<I, U, F> {
        let num_threads = ::num_cpus::get();
        let mut res = Iter {
            pool: CpuPool::new(num_threads),
            queue: VecDeque::new(),
            iter: iter,
            f: Arc::new(f),
            cur_iter: vec![].into_iter(),
        };
        for _ in 0..num_threads * 2 {
            res.spawn();
        }
        res
    }
    fn spawn(&mut self) {
        let future = match self.iter.next() {
            None => return,
            Some(item) => {
                let f = self.f.clone();
                self.pool.spawn_fn(move || Ok(f(item).into_iter().collect()))
            }
        };
        self.queue.push_back(future);
    }
}

impl<I: Iterator, U: IntoIterator, F> Iterator for Iter<I, U, F>
    where F: Fn(I::Item) -> U,
          U::Item: Send + 'static,
          I::Item: Send + 'static,
          F: Sync + Send + 'static
{
    type Item = U::Item;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(item) = self.cur_iter.next() {
                return Some(item);
            }
            let v = match self.queue.pop_front() {
                Some(future) => future.wait().unwrap(),
                None => return None,
            };
            self.cur_iter = v.into_iter();
            self.spawn();
        }
    }
}

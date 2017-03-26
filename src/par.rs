// Copyright (c) 2014-2017 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

//! Parallel iterator for `OsmPbfReader`.

use std::collections::VecDeque;
use {Result, OsmObj};
use futures::Future;
use futures_cpupool::{CpuPool, CpuFuture};

/// A parallel iterator over the `OsmObj` of an `OsmPbfReader`.
pub struct Iter<'a, R: 'a> {
    pool: CpuPool,
    queue: VecDeque<CpuFuture<Vec<Result<OsmObj>>, ()>>,
    blob_iter: ::reader::Blobs<'a, R>,
    obj_iter: ::std::vec::IntoIter<Result<OsmObj>>,
}
impl<'a, R> Iter<'a, R>
    where R: ::std::io::Read
{
    /// Creates a parallel iterator.
    pub fn new(blobs: ::reader::Blobs<'a, R>) -> Iter<R> {
        let num_threads = ::num_cpus::get();
        let mut res = Iter {
            pool: CpuPool::new(num_threads),
            queue: VecDeque::new(),
            blob_iter: blobs,
            obj_iter: vec![].into_iter(),
        };
        for _ in 0..num_threads * 2 {
            res.push_block();
        }
        res
    }
    fn push_block(&mut self) {
        let future = match self.blob_iter.next() {
            None => return,
            Some(blob) => {
                self.pool.spawn_fn(move || Ok(::iter::result_blob_into_iter(blob).collect()))
            }
        };
        self.queue.push_back(future);
    }
}

impl<'a, R> Iterator for Iter<'a, R>
    where R: ::std::io::Read
{
    type Item = Result<OsmObj>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(obj) = self.obj_iter.next() {
                return Some(obj);
            }
            let v = match self.queue.pop_front() {
                Some(f) => f.wait().unwrap(),
                None => return None,
            };
            self.obj_iter = v.into_iter();
            self.push_block();
        }
    }
}

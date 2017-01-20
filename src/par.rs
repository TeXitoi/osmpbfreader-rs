// Copyright (c) 2014-2017 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

//! Parallel iterator for OsmPbfReader.

use ::std::sync::mpsc::{channel, Sender, Receiver};
use ::threadpool::ThreadPool;
use ::{Result, OsmObj};

/// A parallel iterator over the OsmObj of an OsmPbfReader.
pub struct Iter<'a, R: 'a> {
    tx: Option<Sender<Vec<Result<OsmObj>>>>,
    rx: Receiver<Vec<Result<OsmObj>>>,
    pool: ThreadPool,
    blob_iter: ::reader::Blobs<'a, R>,
    obj_iter: ::std::vec::IntoIter<Result<OsmObj>>,
}
impl<'a, R> Iter<'a, R> where R: ::std::io::Read {
    /// Creates a parallel iterator.
    pub fn new<'b>(reader: &'b mut ::reader::OsmPbfReader<R>) -> Iter<'b, R> {
        let num_threads = ::num_cpus::get();
        let (tx, rx) = channel();
        let mut res = Iter {
            tx: Some(tx),
            rx: rx,
            pool: ThreadPool::new(num_threads),
            blob_iter: reader.blobs(),
            obj_iter: vec![].into_iter(),
        };
        for _ in 0..num_threads * 2 {
            res.push_block();
        }
        res
    }
    fn push_block(&mut self) {
        let tx = match self.tx.as_ref() {
            Some(tx) => tx.clone(),
            None => return,
        };
        let blob = match self.blob_iter.next() {
            Some(Ok(b)) => b,
            Some(Err(e)) => {
                tx.send(vec![Err(e)]).unwrap();
                return;
            }
            None => {
                self.tx = None;
                return;
            }
        };
        self.pool.execute(move|| {
            let block = ::reader::primitive_block_from_blob(&blob);
            let block = match block {
                Ok(b) => b,
                Err(e) => {
                    tx.send(vec![Err(e)]).unwrap();
                    return;
                }
            };
            tx.send(::blocks::iter(&block).map(|o| Ok(o)).collect()).unwrap();
        });
    }
}

impl<'a, R> Iterator for Iter<'a, R> where R: ::std::io::Read {
    type Item = Result<OsmObj>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(obj) = self.obj_iter.next() {
                return Some(obj);
            }
            let v = match self.rx.recv() {
                Ok(v) => v,
                Err(_) => return None,
            };
            self.obj_iter = v.into_iter();
            self.push_block();
        }
    }
}

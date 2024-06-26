#![feature(maybe_uninit_uninit_array)]
#![feature(iter_next_chunk)]
#![feature(iter_advance_by)]
#![feature(layout_for_ptr)]
#![feature(const_alloc_layout)]
#![feature(ptr_sub_ptr)]
#![allow(unused)]

use std::fmt::Debug;

pub mod lex;
pub mod parse;
pub mod vm;
pub mod transpile;
pub mod error;
pub mod resolve;

pub use error::Error;

#[derive(Debug, PartialEq)]
pub struct Span<T> where T: Debug {
    pub value: T,
    pub start: u64,
    pub end: u64,
}

impl<T: Debug> Span<T> {
    #[inline]
    pub const fn zero(value: T) -> Span<T> {
        Span {
            value,
            start: 0,
            end: 0,
        }
    }
}

impl<T: Debug> Span<T> {
    #[inline]
    pub fn map<U, F: Fn(T) -> U>(self, mapper: F) -> Span<U> where U: Debug {
        Span {
            value: mapper(self.value),
            start: self.start,
            end: self.end,
        }
    }
}
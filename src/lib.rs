#![no_std]

//! Provides a quality of life macro `cartesian!` to simplify certain loops.
//!
//! The macro takes up to 26 iterators as arguments and creates the cartesian product iterator over
//! all input iterators, kind of like nested for loops.
//!
//! It behaves the same as nested for loops and brings the advantage of
//! being more compact, and simplifies breaking and continuing.
//!
//! # Example
//! ```
//! use cartesian::*;
//!
//! let mut volume_grid = vec![vec![vec![0; 10]; 10]; 10];
//! for (x, y, z) in cartesian!(0..10, 0..10, 0..10) {
//!     volume_grid[x][y][z] = x * y + z;
//! }
//! ```

/// Helper trait implemented for all tuples up to 26 elements to prepend a value to produce a longer tuple
///
/// The implementation is adapted from [this stackoverflow answer](https://stackoverflow.com/a/57454888).
pub trait TuplePrepend<T> {
    type ResultType;
    fn prepend(self, t: T) -> Self::ResultType;
}

impl<T> TuplePrepend<T> for () {
    type ResultType = (T,);
    #[inline]
    fn prepend(self, t: T) -> Self::ResultType {
        (t,)
    }
}

macro_rules! _impl_tuple_prepend {
    ( ()  ) => {};
    ( ($t:ident $( $typ:ident)* )  ) => {
        impl<$t, $($typ,)* TT> TuplePrepend<TT> for ($t, $($typ,)*) {
            type ResultType = (TT, $t, $($typ),*);
            #[inline]
            fn prepend(self, t: TT) -> Self::ResultType {
                #[allow(non_snake_case)]
                let ($t, $($typ,)*) = self;
                (t, $t, $($typ,)*)

            }

        }
        _impl_tuple_prepend!(($($typ)*));

    }

}
_impl_tuple_prepend!((
    A B C D E F G H I J K L M N O P Q R S T U V W X Y Z
));

/// The macro this is all about.
#[macro_export]
macro_rules! cartesian {
    ($iter:expr) => {
        $iter
    };
    ($a:expr, $b:expr) => {
        $a.flat_map(|a| {
            $b.map(move |b| (a, b))
        })
    };
    ($head:expr, $($tail:expr),+) => {
        cartesian!($head, cartesian!($($tail),+)).map(
            |(head, tail)| tail.prepend(head)
        )
    };
}

#[cfg(test)]
extern crate alloc;
#[cfg(test)]
use alloc::{format, string::String, vec};

#[test]
fn two_combination() {
    let mut acc = String::new();

    for (a, b) in cartesian!(0..2, "xy".chars()) {
        acc += &format!("{}{} ", a, b);
    }

    assert_eq!(acc, "0x 0y 1x 1y ");
}

#[test]
fn binary_numbers() {
    let mut acc = String::new();

    let range = 0..2;
    let vec = vec![0, 1];
    let string = vec![String::from("0"), String::from("1")];

    for (a, b, c) in cartesian!(range, vec.iter(), string.iter()) {
        acc += &format!("{}{}{} ", a, b, c);
    }

    assert_eq!(acc, "000 001 010 011 100 101 110 111 ");
}

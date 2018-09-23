// Copyright 2018 Weston Carvalho
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[allow(unused_imports)]
#[macro_use]
extern crate enums_derive;

#[allow(unused_imports)]
#[macro_use]
pub extern crate enum_map;

#[allow(unused_imports)]
#[macro_use]
pub extern crate enum_set;

pub use self::enum_map::*;
pub use self::enum_set::*;

pub use self::enum_map::Enum;
pub use self::enum_set::CLike;

pub use self::enums_derive::*;

use std::iter::Iterator;

pub trait EnumIterator: CLike {
    type Item: EnumIterator;
    type Iter: Iterator;

    fn size() -> usize;
    fn iter() -> Self::Iter;
}

#[cfg(test)]
mod test {
    use CLike;
    use Enum;
    use EnumIterator;

    #[derive(Copy, Clone, PartialEq, Debug)]
    enum Test {
        Hey,
        You,
    }

    /// This is what would be auto-generated for Test if
    /// Test was defined in a crate that imported enum-utils.
    #[automatically_derived]
    impl CLike for Test {
        fn to_u32(&self) -> u32 {
            *self as u32
        }
        unsafe fn from_u32(value: u32) -> Self {
            match value {
                0u32 => Test::Hey,
                1u32 => Test::You,
                _ => unreachable!(),
            }
        }
    }
    #[automatically_derived]
    impl<V> Enum<V> for Test {
        type Array = [V; 2usize];
        const POSSIBLE_VALUES: usize = 2usize;
        #[inline]
        fn slice(array: &Self::Array) -> &[V] {
            array
        }
        #[inline]
        fn slice_mut(array: &mut Self::Array) -> &mut [V] {
            array
        }
        #[inline]
        fn from_usize(value: usize) -> Self {
            match value {
                0usize => Test::Hey,
                1usize => Test::You,
                _ => unreachable!(),
            }
        }
        #[inline]
        fn to_usize(self) -> usize {
            self as usize
        }
        #[inline]
        fn from_function<F: FnMut(Self) -> V>(mut _f: F) -> Self::Array {
            [_f(Test::Hey), _f(Test::You)]
        }
    }
    #[automatically_derived]
    impl EnumIterator for Test {
        type Item = Self;
        type Iter = ::std::slice::Iter<'static, Self::Item>;
        fn size() -> usize {
            2usize
        }
        fn iter() -> Self::Iter {
            const ARRAY: [Test; 2usize] = [Test::Hey, Test::You];
            ARRAY.iter()
        }
    }

    use self::Test::*;
    #[test]
    fn test_map() {
        let map: super::EnumMap<Test, u32> = enum_map![Hey => 4, You => 5];
        for x in Test::iter() {
            match x {
                Hey => assert_eq!(4, map[*x]),
                You => assert_eq!(5, map[*x]),
            }
            println!("{:?} => {:?}", x, map[*x]);
        }
    }

    use std::iter::FromIterator;
    #[test]
    fn test_set() {
        let vec = vec![Hey];
        let set = super::EnumSet::from_iter(vec.into_iter());
        for x in set.iter() {
            assert_eq!(Hey, x);
            println!("{:?}", x);
        }
    }

}

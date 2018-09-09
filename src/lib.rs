
#[allow(unused_imports)]
#[macro_use] extern crate enums_derive;

#[allow(unused_imports)]
#[macro_use] pub extern crate enum_map;

#[allow(unused_imports)]
#[macro_use] pub extern crate enum_set;

pub use self::enum_map::*;
pub use self::enum_set::*;

pub use self::enum_set::CLike;
pub use self::enum_map::Enum;

pub use self::enums_derive::*;

use std::iter::Iterator;

pub trait EnumIterator : CLike {
    type Item: EnumIterator;
    type Iter: Iterator;

    fn size() -> usize;
    fn iter() -> Self::Iter;
}

#[cfg(test)]
mod test {
    use EnumIterator;
    use CLike;
    use Enum;

    #[derive(Copy, Clone, Debug)]
    // #[derive(Copy, Clone, Enums)]
    enum Test {
        Hey,
        You
    }
    # [ automatically_derived ]
    impl CLike for Test {
           fn to_u32 ( & self ) -> u32 { * self as u32 } unsafe fn from_u32 ( value : u32
           ) -> Self {
           match value {
           0u32 => Test :: Hey , 1u32 => Test :: You , _ => unreachable ! (  ) } } } # [
           automatically_derived ] impl < V > Enum < V > for Test {
           type Array = [ V ; 2usize ] ; const POSSIBLE_VALUES : usize = 2usize ; # [
           inline ] fn slice ( array : & Self :: Array ) -> & [ V ] { array } # [ inline
           ] fn slice_mut ( array : & mut Self :: Array ) -> & mut [ V ] { array } # [
           inline ] fn from_usize ( value : usize ) -> Self {
           match value {
           0usize => Test :: Hey , 1usize => Test :: You , _ => unreachable ! (  ) } } #
           [ inline ] fn to_usize ( self ) -> usize { self as usize } # [ inline ] fn
           from_function < F : FnMut ( Self ) -> V > ( mut _f : F ) -> Self :: Array {
           [ _f ( Test :: Hey ) , _f ( Test :: You ) , ] } } # [ automatically_derived ]
           impl EnumIterator for Test {
           type Item = Self ; type Iter = :: std :: slice :: Iter < 'static , Self ::
           Item > ; fn size (  ) -> usize { 2usize } fn iter (  ) -> Self :: Iter {
           const ARRAY : [ Test ; 2usize ] = [ Test :: Hey , Test :: You , ] ; ARRAY .
           iter (  ) } }

    use self::Test::*;
    #[test]
    fn test_macros() {
        let map: super::EnumMap<Test, u32> = enum_map![Hey => 4, You => 5];
        for x in Test::iter() {
            println!("{:?} => {:?}", x, map[*x]);
        }
    }
}
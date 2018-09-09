# About
`enum-utils` is a crate designed to make it easy to deal with C-like enumerated types. It provides the following:
* `trait` **EnumIterator** - Describes an enum whose values can be iterated over.
* `derive` **Enums** - Derives the implementation of three traits for a C-like macro:
    * **Clike** - An `enum` must implement this trait to be used with EnumSet.
    * **EnumIterator** - As described above.
    * **Enum\<V>**  - An `enum` must implement this trait to be used with EnumMap.
* Reexports of [enum_map](https://crates.io/crates/enum-map) and [enum_set](https://crates.io/crates/enum-set), macros included.

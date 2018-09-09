//! Copyright 2018 Weston Carvalho
//! Derived from enum-map-derive, copyright 2017 Konrad Borowshi
#![recursion_limit = "128"]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

use std::iter;

use syn::{Data, DataEnum, DeriveInput, Fields, Ident, Variant};



fn generate_enum_code(name: Ident, data_enum: DataEnum) -> proc_macro2::TokenStream {
    let enum_count = data_enum.variants.len();
    let enum_count_32: u32 = enum_count as u32;
    let mut has_discriminants = false;
    for &Variant {
        ref fields,
        ref discriminant,
        ..
    } in &data_enum.variants
    {
        match *fields {
            Fields::Unit => (),
            _ => return quote!(compile_error!{"#[derive(EnumIter)] requires C style style enum"}),
        }

        if discriminant.is_some() {
            has_discriminants = true;
        }
    }

    let variants_names_a = data_enum.variants.iter().map(|variant| &variant.ident);
    let variants_names_b = data_enum.variants.iter().map(|variant| &variant.ident);
    let variants_names_c = data_enum.variants.iter().map(|variant| &variant.ident);
    let variants_names_d = data_enum.variants.iter().map(|variant| &variant.ident);
    let repeat_name_a = iter::repeat(&name);
    let repeat_name_b = repeat_name_a.clone();
    let repeat_name_c = repeat_name_a.clone();
    let repeat_name_d = repeat_name_a.clone();
    let counter = 0..enum_count;
    let counter32 = 0..enum_count_32;

    let (to_usize, to_u32) = if enum_count == 0 || has_discriminants {
        let variants_names = data_enum.variants.iter().map(|variant| &variant.ident);
        let repeat_name = repeat_name_a.clone();
        let counter = counter.clone();
        let result = quote! {
                match self {
                    #(
                        #repeat_name::#variants_names => #counter,
                    )*
                }
            };
        (result.clone(), result)

    } else {
        (
            quote! { self as usize },
            quote! { *self as u32 }
        )
    };

    let (from_usize, from_u32) = (
        quote! {
            match value {
                #(
                    #counter => #repeat_name_a::#variants_names_a,
                )*
                _ => unreachable!()
            }
        },
        quote! {
            match value {
                #(
                    #counter32 => #repeat_name_d::#variants_names_d,
                )*
                _ => unreachable!()
            }
        }
    );
    
    quote! {
        #[automatically_derived]
        impl ::enum_utils::CLike for #name {

            fn to_u32(&self) -> u32 {
                #to_u32
            }

            unsafe fn from_u32(value: u32) -> Self {
                #from_u32
            }
        }

        #[automatically_derived]
        impl<V> ::enum_utils::Enum<V> for #name {
            type Array = [V; #enum_count];
            const POSSIBLE_VALUES: usize = #enum_count;

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
                #from_usize
            }

            #[inline]
            fn to_usize(self) -> usize {
                #to_usize
            }

            #[inline]
            fn from_function<F: FnMut(Self) -> V>(mut _f: F) -> Self::Array {
                [#(
                    _f(#repeat_name_b::#variants_names_b),
                )*]
            }
        }


        #[automatically_derived]
        impl ::enum_utils::EnumIterator for #name {
            type Item = Self;
            type Iter = ::std::slice::Iter<'static, Self::Item>;

            fn size() -> usize {
                #enum_count
            }

            fn iter() -> Self::Iter {
                const ARRAY: [#name; #enum_count] = [#(
                    #repeat_name_c::#variants_names_c,
                )*];
                ARRAY.iter()
            }
        }
    }
}

#[proc_macro_derive(Enums)]
pub fn derive_enum_map(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();

    let result = match input.data {
        Data::Enum(data_enum) => generate_enum_code(input.ident, data_enum),
        _ => quote!(compile_error!{"#[derive(EnumIter)] is only defined for enum_utils"}),
    };

    result.into()
}

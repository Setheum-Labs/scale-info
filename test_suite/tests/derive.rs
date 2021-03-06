// Copyright 2019-2020 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::boxed::Box;

use pretty_assertions::assert_eq;
use scale_info::build::*;
use scale_info::{tuple_meta_type, Path, Type, TypeInfo};

fn assert_type<T, E>(expected: E)
where
	T: TypeInfo + ?Sized,
	E: Into<Type>,
{
	assert_eq!(T::type_info(), expected.into());
}

macro_rules! assert_type {
	( $ty:ty, $expected:expr ) => {{
		assert_type::<$ty, _>($expected)
		}};
}

#[test]
fn struct_derive() {
	#[allow(unused)]
	#[derive(TypeInfo)]
	struct S<T, U> {
		pub t: T,
		pub u: U,
	}

	let struct_type = Type::builder()
		.path(Path::new("S", "derive"))
		.type_params(tuple_meta_type!(bool, u8))
		.composite(Fields::named().field_of::<bool>("t").field_of::<u8>("u"));

	assert_type!(S<bool, u8>, struct_type);

	// With "`Self` typed" fields

	type SelfTyped = S<Box<S<bool, u8>>, bool>;

	let self_typed_type = Type::builder()
		.path(Path::new("S", "derive"))
		.type_params(tuple_meta_type!(Box<S<bool, u8>>, bool))
		.composite(Fields::named().field_of::<Box<S<bool, u8>>>("t").field_of::<bool>("u"));
	assert_type!(SelfTyped, self_typed_type);
}

#[test]
fn tuple_struct_derive() {
	#[allow(unused)]
	#[derive(TypeInfo)]
	struct S<T>(T);

	let ty = Type::builder()
		.path(Path::new("S", "derive"))
		.type_params(tuple_meta_type!(bool))
		.composite(Fields::unnamed().field_of::<bool>());

	assert_type!(S<bool>, ty);
}

#[test]
fn unit_struct_derive() {
	#[allow(unused)]
	#[derive(TypeInfo)]
	struct S;

	let ty = Type::builder().path(Path::new("S", "derive")).composite(Fields::unit());

	assert_type!(S, ty);
}

#[test]
fn c_like_enum_derive() {
	#[allow(unused)]
	#[derive(TypeInfo)]
	enum E {
		A,
		B = 10,
	}

	let ty = Type::builder()
		.path(Path::new("E", "derive"))
		.variant(Variants::fieldless().variant("A", 0u64).variant("B", 10u64));

	assert_type!(E, ty);
}

#[test]
fn enum_derive() {
	#[allow(unused)]
	#[derive(TypeInfo)]
	enum E<T> {
		A(T),
		B { b: T },
		C,
	}

	let ty = Type::builder()
		.path(Path::new("E", "derive"))
		.type_params(tuple_meta_type!(bool))
		.variant(
			Variants::with_fields()
				.variant("A", Fields::unnamed().field_of::<bool>())
				.variant("B", Fields::named().field_of::<bool>("b"))
				.variant_unit("C"),
		);

	assert_type!(E<bool>, ty);
}

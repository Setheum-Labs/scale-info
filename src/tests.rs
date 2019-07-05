// Copyright 2019
//     by  Centrality Investments Ltd.
//     and Parity Technologies (UK) Ltd.
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

use crate::*;
use std::marker::PhantomData;

fn assert_type_id<T, E>(expected: E)
where
	T: HasTypeId + ?Sized,
	E: Into<TypeId>,
{
	assert_eq!(T::type_id(), expected.into());
}

macro_rules! assert_type_id {
	( $ty:ty, $expected:expr ) => {{
		assert_type_id::<$ty, _>($expected)
		}};
}

#[test]
fn primitives() {
	assert_type_id!(bool, TypeIdPrimitive::Bool);
	assert_type_id!(String, TypeIdPrimitive::Str);
	assert_type_id!(&str, TypeIdPrimitive::Str);
	assert_type_id!(i8, TypeIdPrimitive::I8);

	assert_type_id!(Box<String>, TypeIdPrimitive::Str);
	assert_type_id!(&String, TypeIdPrimitive::Str);
	assert_type_id!([bool], TypeIdSlice::new(TypeIdPrimitive::Bool));
	assert_type_id!(PhantomData<bool>, TypeIdPrimitive::Bool);
}

#[test]
fn prelude_items() {
	assert_type_id!(
		Option<u128>,
		TypeIdCustom::new("Option", Namespace::prelude(), tuple_type_id!(u128))
	);
	assert_type_id!(
		Result<bool, String>,
		TypeIdCustom::new("Result", Namespace::prelude(), tuple_type_id!(bool, str))
	);
}

#[test]
fn tuple_primitives() {
	// unit
	assert_type_id!((), TypeIdTuple::new(tuple_type_id!()));

	// tuple with one element
	assert_type_id!((bool,), TypeIdTuple::new(tuple_type_id!(bool)));

	// tuple with multiple elements
	assert_type_id!((bool, String), TypeIdTuple::new(tuple_type_id!(bool, String)));

	// nested tuple
	assert_type_id!(
		((i8, i16), (u32, u64)),
		TypeIdTuple::new(vec![
			TypeIdTuple::new(tuple_type_id!(i8, i16)).into(),
			TypeIdTuple::new(tuple_type_id!(u32, u64)).into(),
		])
	);
}

#[test]
fn array_primitives() {
	// array
	assert_type_id!([bool; 3], TypeIdArray::new(3, bool::type_id()));
	// nested
	assert_type_id!([[i32; 5]; 5], TypeIdArray::new(5, TypeIdArray::new(5, i32::type_id())));
	// slice
	assert_type_id!([bool], TypeIdSlice::new(bool::type_id()));
	// vec
	assert_type_id!(
		Vec<bool>,
		TypeIdCustom::new("Vec", Namespace::prelude(), tuple_type_id![bool])
	);
}

#[test]
fn struct_with_generics() {
	#[allow(unused)]
	struct MyStruct<T> {
		data: T,
	}

	impl<T> HasTypeId for MyStruct<T>
	where
		T: HasTypeId,
	{
		fn type_id() -> TypeId {
			TypeIdCustom::new(
				"MyStruct",
				Namespace::from_str(module_path!()).unwrap(),
				tuple_type_id!(T),
			)
			.into()
		}
	}

	impl<T> HasTypeDef for MyStruct<T>
	where
		T: Metadata,
	{
		fn type_def() -> TypeDef {
			TypeDefStruct::new(vec![NamedField::new("data", T::type_id())]).into()
		}
	}

	impl<T> RegisterSubtypes for MyStruct<T>
	where
		T: Metadata,
	{
		fn register_subtypes(registry: &mut Registry) {
			registry.register_type::<T>();
		}
	}

	// Normal struct
	let struct_bool_id = TypeIdCustom::new(
		"MyStruct",
		Namespace::new(vec!["type_metadata", "tests"]).unwrap(),
		tuple_type_id!(bool),
	);
	assert_type_id!(MyStruct<bool>, struct_bool_id.clone());

	let struct_bool_def = TypeDefStruct::new(vec![NamedField::new("data", bool::type_id())]).into();
	assert_eq!(<MyStruct<bool>>::type_def(), struct_bool_def);

	// With "`Self` typed" fields
	type SelfTyped = MyStruct<Box<MyStruct<bool>>>;
	let expected_type_id = TypeIdCustom::new(
		"MyStruct",
		Namespace::new(vec!["type_metadata", "tests"]).unwrap(),
		vec![struct_bool_id.clone().into()],
	);
	assert_type_id!(SelfTyped, expected_type_id);
	assert_eq!(
		SelfTyped::type_def(),
		TypeDefStruct::new(vec![NamedField::new("data", struct_bool_id.clone()),]).into(),
	);
}

#[test]
fn struct_derive() {
	use crate as type_metadata;
	use type_metadata_derive::{TypeId, TypeDef};

	#[allow(unused)]
	#[derive(TypeId, TypeDef)]
	struct S<T, U> {
		pub t: T,
		pub u: U,
	}

	let type_id = TypeIdCustom::new(
		"S",
		Namespace::new(vec!["type_metadata", "tests"]).unwrap(),
		tuple_type_id!(bool, u8),
	);
	assert_type_id!(S<bool, u8>, type_id);

	let type_def = TypeDefStruct::new(vec![
		NamedField::new("t", bool::type_id()),
		NamedField::new("u", u8::type_id()),
	]).into();
	assert_eq!(<S<bool, u8>>::type_def(), type_def);
}

#[test]
fn tuple_struct_derive() {
	use crate as type_metadata;
	use type_metadata_derive::{TypeId, TypeDef};

	#[allow(unused)]
	#[derive(TypeId, TypeDef)]
	struct S<T>(T);

	let type_id = TypeIdCustom::new(
		"S",
		Namespace::new(vec!["type_metadata", "tests"]).unwrap(),
		tuple_type_id!(bool),
	);
	assert_type_id!(S<bool>, type_id);

	let type_def = TypeDefTupleStruct::new(vec![UnnamedField::new::<bool>()]).into();
	assert_eq!(<S<bool>>::type_def(), type_def);
}

#[test]
fn unit_struct_derive() {
	use crate as type_metadata;
	use type_metadata_derive::{TypeId, TypeDef};

	#[allow(unused)]
	#[derive(TypeId, TypeDef)]
	struct S;

	let type_id = TypeIdCustom::new(
		"S",
		Namespace::new(vec!["type_metadata", "tests"]).unwrap(),
		vec![],
	);
	assert_type_id!(S, type_id);

	let type_def = TypeDefTupleStruct::unit().into();
	assert_eq!(S::type_def(), type_def);
}

#[test]
fn c_like_enum_derive() {
	use crate as type_metadata;
	use type_metadata_derive::{TypeId, TypeDef};

	#[allow(unused)]
	#[derive(TypeId, TypeDef)]
	enum E {
		A,
		B = 10,
	}

	let type_id = TypeIdCustom::new(
		"E",
		Namespace::new(vec!["type_metadata", "tests"]).unwrap(),
		vec![],
	);
	assert_type_id!(E, type_id);

	let type_def = TypeDefClikeEnum::new(vec![
		ClikeEnumVariant::new("A", 0u64),
		ClikeEnumVariant::new("B", 10u64),
	]).into();
	assert_eq!(E::type_def(), type_def);
}


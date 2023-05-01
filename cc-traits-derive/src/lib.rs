// #![deny(missing_docs)]

use proc_macro2::{Literal, Span};
use quote::{quote, quote_spanned, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{
	parse_macro_input, parse_quote, AttrStyle, DeriveInput, File, GenericParam, Generics, Ident,
	LifetimeParam, MacroDelimiter, Path, Token, Type, TypeParam, TypePath, WherePredicate,
};

fn root() -> Ident {
	let crate_name =
		proc_macro_crate::crate_name("cc-traits").expect("`cc-traits` not found in `Cargo.toml`");
	match crate_name {
		proc_macro_crate::FoundCrate::Itself => Ident::new("crate", Span::call_site()),
		proc_macro_crate::FoundCrate::Name(name) => Ident::new(&name, Span::call_site()),
	}
}

fn compile_error(spanned: impl Spanned, msg: impl AsRef<str>) -> proc_macro::TokenStream {
	let msg_lit = Literal::string(msg.as_ref());
	quote_spanned! {spanned.span()=>
		::std::compile_error!(#msg_lit);
	}
	.into()
}

macro_rules! syn_unwrap {
	($e:expr) => {
		match $e {
			Ok(item) => item,
			Err(err) => return err.to_compile_error().into(),
		}
	};
}

fn generics_key_param(generics: &Generics) -> Option<&TypeParam> {
	generics.type_params().next()
}

fn generics_key_ident(generics: &Generics) -> Option<&Ident> {
	generics_key_param(generics).map(|type_param| &type_param.ident)
}

fn generics_item_param(generics: &Generics) -> Option<&TypeParam> {
	let mut type_params = generics.type_params();
	let first = type_params.next();
	let second = type_params.next();
	second.or(first)
}
fn generics_item_ident(generics: &Generics) -> Option<&Ident> {
	generics_item_param(generics).map(|type_param| &type_param.ident)
}

#[proc_macro]
pub fn derive_external(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let file = parse_macro_input!(input as File);
	if file.shebang.is_some() {
		return compile_error(file.shebang, "Unexpected shebang.");
	};
	if let Some(attr) = file.attrs.first() {
		return compile_error(attr, "Unexpected outer attribute.");
	}
	file.items
		.into_iter()
		.map(|item| {
			let item: DeriveInput = syn_unwrap!(syn::parse2(item.to_token_stream()));
			item.attrs
				.iter()
				.flat_map(|attr| {
					assert_eq!(AttrStyle::Outer, attr.style);
					let Some("derive") = attr.path().get_ident().map(ToString::to_string).as_deref() else {
						return compile_error(attr, "Item in `derive_external!` should only have `#[derive]` attributes.");
					};
					let derive_list = syn_unwrap!(attr.meta.require_list());
					assert!(matches!(derive_list.delimiter, MacroDelimiter::Paren(..)));
					let traits = syn_unwrap!(derive_list
						.parse_args_with(|parse_stream: ParseStream| parse_stream
							.parse_terminated(Ident::parse, Token![,])));
					traits
						.into_iter()
						.map(|trate| match &*trate.to_string() {
							"Collection" => collection(&item),
							"CollectionRef" => collection_ref(&item),
							"CollectionMut" => collection_mut(&item),
							"SimpleCollectionRef" => simple_collection_ref(&item),
							"SimpleCollectionMut" => simple_collection_mut(&item),
							"Keyed" => keyed(&item),
							"KeyedRef" => keyed_ref(&item),
							"SimpleKeyedRef" => simple_keyed_ref(&item),
							"Len" => len(&item),
							"Get" => get(&item),
							"GetMut" => get_mut(&item),
							unknown => return compile_error(unknown, "Cannot derive unknown trait. This macro only works for traits from cc-traits."),
						})
						.collect()
				})
				.collect::<proc_macro::TokenStream>()
		})
		.collect()
}

#[proc_macro_derive(Collection)]
pub fn derive_collection(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	collection(&parse_macro_input!(input as DeriveInput))
}
fn collection(item: &DeriveInput) -> proc_macro::TokenStream {
	let root = root();
	let name = &item.ident;
	let (impl_generics, type_generics, where_clause) = item.generics.split_for_impl();
	let Some(item_ident) = generics_item_ident(&item.generics) else {
		return compile_error(item, "`Collection`: Item must have at least one generic argument, to be used as the `Item`.");
	};

	quote! {
		impl #impl_generics #root::Collection for #name #type_generics #where_clause {
			type Item = #item_ident;
		}
	}
	.into()
}

#[proc_macro_derive(CollectionRef)]
pub fn derive_collection_ref(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	collection_ref(&parse_macro_input!(input as DeriveInput))
}
fn collection_ref(item: &DeriveInput) -> proc_macro::TokenStream {
	let root = root();
	let name = &item.ident;
	let (impl_generics, type_generics, where_clause) = item.generics.split_for_impl();
	let Some(item_ident) = generics_item_ident(&item.generics) else {
		return compile_error(item, "`CollectionRef`: Item must have at least one generic argument, to be used as the `Item`.");
	};

	quote! {
		impl #impl_generics #root::CollectionRef for #name #type_generics #where_clause {
			type ItemRef<'a> = &'a #item_ident where Self: 'a;

			#root::covariant_item_ref!();
		}
	}
	.into()
}

#[proc_macro_derive(CollectionMut)]
pub fn derive_collection_mut(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	collection_mut(&parse_macro_input!(input as DeriveInput))
}
fn collection_mut(item: &DeriveInput) -> proc_macro::TokenStream {
	let root = root();
	let name = &item.ident;
	let (impl_generics, type_generics, where_clause) = item.generics.split_for_impl();
	let Some(item_ident) = generics_item_ident(&item.generics) else {
		return compile_error(item, "`CollectionMut`: Item must have at least one generic argument, to be used as the `Item`.");
	};

	quote! {
		impl #impl_generics #root::CollectionMut for #name #type_generics #where_clause {
			type ItemMut<'a> = &'a mut #item_ident where Self: 'a;

			#root::covariant_item_mut!();
		}
	}
	.into()
}

#[proc_macro_derive(SimpleCollectionMut)]
pub fn derive_simple_collection_mut(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	simple_collection_mut(&parse_macro_input!(input as DeriveInput))
}
fn simple_collection_mut(item: &DeriveInput) -> proc_macro::TokenStream {
	let root = root();
	let name = &item.ident;
	let (impl_generics, type_generics, where_clause) = item.generics.split_for_impl();

	quote! {
		impl #impl_generics #root::SimpleCollectionMut for #name #type_generics #where_clause {
			#root::simple_collection_mut!();
		}
	}
	.into()
}

#[proc_macro_derive(SimpleCollectionRef)]
pub fn derive_simple_collection_ref(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	simple_collection_ref(&parse_macro_input!(input as DeriveInput))
}
fn simple_collection_ref(item: &DeriveInput) -> proc_macro::TokenStream {
	let root = root();
	let name = &item.ident;
	let (impl_generics, type_generics, where_clause) = item.generics.split_for_impl();

	quote! {
		impl #impl_generics #root::SimpleCollectionRef for #name #type_generics #where_clause {
			#root::simple_collection_ref!();
		}
	}
	.into()
}

#[proc_macro_derive(Keyed)]
pub fn derive_keyed(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	keyed(&parse_macro_input!(input as DeriveInput))
}
fn keyed(item: &DeriveInput) -> proc_macro::TokenStream {
	let root = root();
	let name = &item.ident;
	let (impl_generics, type_generics, where_clause) = item.generics.split_for_impl();
	let Some(key_ident) = generics_key_ident(&item.generics) else {
		return compile_error(item, "`Keyed`: Item must have at least one generic argument, to be used as the `Key`.");
	};

	quote! {
		impl #impl_generics #root::Keyed for #name #type_generics #where_clause {
			type Key = #key_ident;
		}
	}
	.into()
}

#[proc_macro_derive(KeyedRef)]
pub fn derive_keyed_ref(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	keyed_ref(&parse_macro_input!(input as DeriveInput))
}
fn keyed_ref(item: &DeriveInput) -> proc_macro::TokenStream {
	let root = root();
	let name = &item.ident;
	let (impl_generics, type_generics, where_clause) = item.generics.split_for_impl();
	let Some(key_ident) = generics_key_ident(&item.generics) else {
		return compile_error(item, "`KeyedRef`: Item must have at least one generic argument, to be used as the `Key`.");
	};

	quote! {
		impl #impl_generics #root::KeyedRef for #name #type_generics #where_clause {
			type KeyRef<'a> = &'a #key_ident where Self: 'a;

			#root::covariant_key_ref!();
		}
	}
	.into()
}

#[proc_macro_derive(SimpleKeyedRef)]
pub fn derive_simple_keyed_ref(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	simple_keyed_ref(&parse_macro_input!(input as DeriveInput))
}
fn simple_keyed_ref(item: &DeriveInput) -> proc_macro::TokenStream {
	let root = root();
	let name = &item.ident;
	let (impl_generics, type_generics, where_clause) = item.generics.split_for_impl();

	quote! {
		impl #impl_generics #root::SimpleKeyedRef for #name #type_generics #where_clause {
			#root::simple_keyed_ref!();
		}
	}
	.into()
}

#[proc_macro_derive(Len)]
pub fn derive_len(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	len(&parse_macro_input!(input as DeriveInput))
}
fn len(item: &DeriveInput) -> proc_macro::TokenStream {
	let root = root();
	let name = &item.ident;
	let (impl_generics, type_generics, where_clause) = item.generics.split_for_impl();

	quote! {
		impl #impl_generics #root::Len for #name #type_generics #where_clause {
			#[inline(always)]
			fn len(&self) -> usize {
				self.len()
			}

			#[inline(always)]
			fn is_empty(&self) -> bool {
				self.is_empty()
			}
		}
	}
	.into()
}

#[proc_macro_derive(Get)]
pub fn derive_get(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	get(&parse_macro_input!(input as DeriveInput))
}
fn get(item: &DeriveInput) -> proc_macro::TokenStream {
	let root = root();
	let name = &item.ident;
	let (_impl_generics, type_generics, where_clause) = item.generics.split_for_impl();

	let Some(key_param) = generics_key_param(&item.generics) else {
		return compile_error(item, "`Get`: Item must have at least one generic argument, to be used as the `Key`.");
	};

	let params = &item.generics.params;
	let predicates = where_clause.map(|wc| &wc.predicates);
	let key_type = Type::Path(TypePath {
		qself: None,
		path: key_param.ident.clone().into(),
	});
	let key_bounds = &key_param.bounds;
	let key_predicates = predicates
		.into_iter()
		.flatten()
		.flat_map(|pred| match pred {
			WherePredicate::Lifetime(_) => None,
			WherePredicate::Type(pred_type) => Some(pred_type),
			_ => panic!(),
		})
		.filter(|&pred_type| &key_type == &pred_type.bounded_ty)
		.cloned()
		.map(|mut pred_type| {
			pred_type.bounded_ty = parse_quote!(__Q);
			pred_type
		});
	let mut augmented_generics: Generics = parse_quote!(<'__a, __Q: #key_bounds, #params>);
	augmented_generics.where_clause = parse_quote! {
		where
			#key_type: ::std::borrow::Borrow<__Q>,
			__Q: ?Sized,
			#( #key_predicates, )*
			#predicates
	};
	let (impl_generics, _type_generics, where_clause) = augmented_generics.split_for_impl();

	quote! {
		impl #impl_generics #root::Get<&'__a __Q> for #name #type_generics #where_clause
		{
			#[inline(always)]
			fn get(&self, key: &'__a __Q) -> Option<Self::ItemRef<'_>> {
				self.get(key)
			}
		}
	}
	.into()
}

#[proc_macro_derive(GetMut)]
pub fn derive_get_mut(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	get_mut(&parse_macro_input!(input as DeriveInput))
}
fn get_mut(item: &DeriveInput) -> proc_macro::TokenStream {
	let root = root();
	let name = &item.ident;
	let (_impl_generics, type_generics, where_clause) = item.generics.split_for_impl();

	let Some(key_param) = generics_key_param(&item.generics) else {
		return compile_error(item, "`GetMut`: Item must have at least one generic argument, to be used as the `Key`.");
	};

	let params = &item.generics.params;
	let predicates = where_clause.map(|wc| &wc.predicates);
	let key_type = Type::Path(TypePath {
		qself: None,
		path: key_param.ident.clone().into(),
	});
	let key_bounds = &key_param.bounds;
	let key_predicates = predicates
		.into_iter()
		.flatten()
		.flat_map(|pred| match pred {
			WherePredicate::Lifetime(_) => None,
			WherePredicate::Type(pred_type) => Some(pred_type),
			_ => panic!(),
		})
		.filter(|&pred_type| &key_type == &pred_type.bounded_ty)
		.cloned()
		.map(|mut pred_type| {
			pred_type.bounded_ty = parse_quote!(__Q);
			pred_type
		});
	let mut augmented_generics: Generics = parse_quote!(<'__a, __Q: #key_bounds, #params>);
	augmented_generics.where_clause = parse_quote! {
		where
			#key_type: ::std::borrow::Borrow<__Q>,
			__Q: ?Sized,
			#( #key_predicates, )*
			#predicates
	};
	let (impl_generics, _type_generics, where_clause) = augmented_generics.split_for_impl();

	quote! {
		impl #impl_generics #root::GetMut<&'__a __Q> for #name #type_generics #where_clause
		{
			#[inline(always)]
			fn get_mut(&mut self, key: &'__a __Q) -> Option<Self::ItemMut<'_>> {
				self.get_mut(key)
			}
		}
	}
	.into()
}

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, ItemEnum};
use uuid::Uuid;

#[proc_macro_derive(EnumUuid)]
pub fn derive_answer_key(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let enum_repr = parse2::<ItemEnum>(TokenStream::from(tokens)).expect("Expected an enum");
	let name_ident = enum_repr.ident;
	let mut from_parts = Vec::new();
	let mut to_parts = Vec::new();

	for variant in enum_repr.variants.iter() {
		let variant_ident = &variant.ident;
		let uuid = Uuid::new_v4().to_string();

		from_parts.push(quote! {
			if id == uuid::uuid!(#uuid) {
				return Some(#name_ident::#variant_ident)
			}
		});

		to_parts.push(quote! {
			#name_ident::#variant_ident => uuid::uuid!(#uuid),
		});
	}

	proc_macro::TokenStream::from(quote! {
		impl enum_uuid::EnumUuid for #name_ident {
			fn from_id(id: uuid::Uuid) -> Option<Self> {
				#( #from_parts )*

				None
			}

			fn to_id(self) -> uuid::Uuid {
				match self {
					#( #to_parts )*
				}
			}
		}
	})
}

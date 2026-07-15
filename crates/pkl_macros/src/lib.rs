//! `pkl_macros` — Derive macros for `pkl_core`.
//!
//! Provides `#[derive(PklDecode)]` for automatic deserialization of Pkl values
//! into Rust structs and enums.
//!
//! # Struct attributes
//!
//! Apply `#[pkl(...)]` to struct fields:
//!
//! | Attribute | Description |
//! |-----------|-------------|
//! | `#[pkl(rename = "name")]` | Override the Pkl property name |
//! | `#[pkl(default)]` | Use `Default::default()` if missing |
//! | `#[pkl(skip)]` | Skip this field entirely |
//! | `#[pkl(flatten)]` | Capture remaining props into `BTreeMap<String, Value>` |
//!
//! # Enum attributes
//!
//! Apply `#[pkl(tag = "field")]` to the enum declaration for tagged unions.
//! The tag field is read from the Pkl object to dispatch to the correct variant.
//!
//! Apply `#[pkl(tag = "value")]` to individual variants to override the tag value
//! (defaults to the variant name in camelCase).
//!
//! # Examples
//!
//! ```rust,ignore
//! use pkl_core::PklDecode;
//!
//! // Simple struct with snake_case → camelCase conversion
//! #[derive(PklDecode)]
//! struct Config {
//!     db_host: String,     // reads Pkl property "dbHost"
//!     max_connections: u16, // reads "maxConnections"
//! }
//!
//! // Optional fields
//! #[derive(PklDecode)]
//! struct Server {
//!     host: String,
//!     #[pkl(default)]
//!     port: u16,            // defaults to 0 if missing
//! }
//!
//! // Flatten: capture remaining properties
//! #[derive(PklDecode)]
//! struct Flexible {
//!     name: String,
//!     #[pkl(flatten)]
//!     extra: std::collections::BTreeMap<String, pkl_core::Value>,
//! }
//!
//! // Tagged union
//! #[derive(PklDecode)]
//! #[pkl(tag = "type")]
//! enum Shape {
//!     #[pkl(tag = "circle")]
//!     Circle { radius: f64 },
//!     #[pkl(tag = "rect")]
//!     Rect { width: f64, height: f64 },
//! }
//! ```

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Meta, Expr, Lit};
use quote::quote;

#[proc_macro_derive(PklDecode, attributes(pkl))]
pub fn derive_pkl_decode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    match &input.data {
        Data::Struct(data) => derive_struct(name, &data.fields),
        Data::Enum(data) => derive_enum(name, &input.attrs, data),
        _ => panic!("PklDecode only supports structs and enums"),
    }
}

// ── Struct derive ──

fn derive_struct(name: &syn::Ident, fields: &Fields) -> TokenStream {
    let named_fields = match fields {
        Fields::Named(f) => f,
        _ => panic!("PklDecode requires named fields for structs"),
    };

    let mut field_names = Vec::new();
    let mut field_types = Vec::new();
    let mut pkl_names = Vec::new();
    let mut has_defaults = Vec::new();
    let mut skips = Vec::new();
    let mut flattens = Vec::new();

    for field in &named_fields.named {
        let field_name = field.ident.as_ref().unwrap();
        let field_name_str = field_name.to_string();
        let mut pkl_name = snake_to_camel(&field_name_str);
        let mut has_default = false;
        let mut skip = false;
        let mut flatten = false;

        for attr in &field.attrs {
            if attr.path().is_ident("pkl") {
                parse_field_attr(attr, &mut pkl_name, &mut has_default, &mut skip, &mut flatten);
            }
        }

        field_names.push(field_name.clone());
        field_types.push(field.ty.clone());
        pkl_names.push(pkl_name);
        has_defaults.push(has_default);
        skips.push(skip);
        flattens.push(flatten);
    }

    let field_assignments: Vec<_> = (0..field_names.len()).map(|i| {
        let name = &field_names[i];
        let ty = &field_types[i];
        let pkl_name = &pkl_names[i];
        let has_default = has_defaults[i];
        let skip = skips[i];
        let flatten = flattens[i];

        if skip {
            return quote! { #name: ::core::default::Default::default() };
        }
        if flatten {
            return quote! { #name: props.clone() };
        }
        if has_default {
            quote! {
                #name: match props.remove(#pkl_name) {
                    Some(__val) => <#ty as ::pkl_core::PklDecode>::decode(__val)?,
                    None => ::core::default::Default::default(),
                }
            }
        } else {
            quote! {
                #name: <#ty as ::pkl_core::PklDecode>::decode(
                    props.remove(#pkl_name)
                        .ok_or_else(|| ::pkl_core::PklError::MissingProperty(#pkl_name.to_string()))?
                )?
            }
        }
    }).collect();

    let expanded = quote! {
        impl ::pkl_core::PklDecode for #name {
            fn decode(__pkl_value: ::pkl_core::Value) -> ::std::result::Result<Self, ::pkl_core::PklError> {
                let mut props = match __pkl_value {
                    ::pkl_core::Value::Object(map) => map,
                    other => return Err(::pkl_core::PklError::TypeMismatch {
                        expected: stringify!(#name),
                        actual: format!("{:?}", other),
                    }),
                };
                Ok(Self { #(#field_assignments,)* })
            }
        }
    };
    TokenStream::from(expanded)
}

// ── Enum derive ──

fn derive_enum(name: &syn::Ident, attrs: &[syn::Attribute], data: &syn::DataEnum) -> TokenStream {
    let tag_field = parse_enum_tag_attr(attrs);
    if let Some(tag) = tag_field {
        derive_tagged_enum(name, &tag, data)
    } else {
        derive_untagged_enum(name, data)
    }
}

fn derive_untagged_enum(name: &syn::Ident, data: &syn::DataEnum) -> TokenStream {
    let variant_arms: Vec<_> = data.variants.iter().map(|variant| {
        let var_name = &variant.ident;
        let var_str = var_name.to_string();
        match &variant.fields {
            Fields::Unit => quote! {
                if let ::pkl_core::Value::String(__s) = &__pkl_value {
                    if __s == #var_str { return Ok(Self::#var_name); }
                }
            },
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                let inner_ty = &fields.unnamed[0].ty;
                quote! {
                    if let Ok(__inner) = <#inner_ty as ::pkl_core::PklDecode>::decode(__pkl_value.clone()) {
                        return Ok(Self::#var_name(__inner));
                    }
                }
            }
            Fields::Named(_) => {
                quote! {
                    if let Ok(__inner) = <Self as ::pkl_core::PklDecode>::decode(__pkl_value.clone()) {
                        return Ok(__inner);
                    }
                }
            }
            _ => quote!{}
        }
    }).collect();

    let expanded = quote! {
        impl ::pkl_core::PklDecode for #name {
            fn decode(__pkl_value: ::pkl_core::Value) -> ::std::result::Result<Self, ::pkl_core::PklError> {
                #(#variant_arms)*
                Err(::pkl_core::PklError::DecodeError(
                    format!("no enum variant matched for {}: {:?}", stringify!(#name), __pkl_value)
                ))
            }
        }
    };
    TokenStream::from(expanded)
}

fn derive_tagged_enum(name: &syn::Ident, tag_field: &str, data: &syn::DataEnum) -> TokenStream {
    let variant_arms: Vec<_> = data.variants.iter().map(|variant| {
        let var_name = &variant.ident;
        let var_str = var_name.to_string();

        let mut tag_value = snake_to_camel(&var_str);
        for attr in &variant.attrs {
            if attr.path().is_ident("pkl") {
                if let Meta::List(list) = &attr.meta {
                    if let Ok(nested) = list.parse_args_with(
                        syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated
                    ) {
                        for m in nested {
                            if let Meta::NameValue(nv) = &m {
                                if nv.path.is_ident("tag") {
                                    if let Expr::Lit(expr_lit) = &nv.value {
                                        if let Lit::Str(s) = &expr_lit.lit {
                                            tag_value = s.value();
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        let tag_str = tag_value.clone();

        match &variant.fields {
            Fields::Unit => quote! { #tag_str => Ok(Self::#var_name), },
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                let inner_ty = &fields.unnamed[0].ty;
                quote! {
                    #tag_str => {
                        let __inner = <#inner_ty as ::pkl_core::PklDecode>::decode(__pkl_value.clone())?;
                        Ok(Self::#var_name(__inner))
                    }
                }
            }
            Fields::Named(fields) => {
                let mut field_tokens = Vec::new();
                for f in &fields.named {
                    let fname = f.ident.as_ref().unwrap();
                    let fty = &f.ty;
                    let pkl_name = snake_to_camel(&fname.to_string());
                    let err_msg = format!("missing field `{}`", pkl_name);
                    field_tokens.push(quote! {
                        #fname: <#fty as ::pkl_core::PklDecode>::decode(
                            __map.remove(#pkl_name)
                                .ok_or_else(|| ::pkl_core::PklError::MissingProperty(#err_msg.to_string()))?
                        )?
                    });
                }
                quote! {
                    #tag_str => {
                        let mut __map = match __pkl_value {
                            ::pkl_core::Value::Object(m) => m,
                            _ => return Err(::pkl_core::PklError::TypeMismatch {
                                expected: "object", actual: format!("{:?}", __pkl_value),
                            }),
                        };
                        Ok(Self::#var_name { #(#field_tokens,)* })
                    }
                }
            }
            _ => quote!{}
        }
    }).collect();

    let expanded = quote! {
        impl ::pkl_core::PklDecode for #name {
            fn decode(__pkl_value: ::pkl_core::Value) -> ::std::result::Result<Self, ::pkl_core::PklError> {
                let __props = match &__pkl_value {
                    ::pkl_core::Value::Object(map) => map,
                    other => return Err(::pkl_core::PklError::TypeMismatch {
                        expected: "tagged object",
                        actual: format!("{:?}", other),
                    }),
                };
                let __tag = match __props.get(#tag_field) {
                    Some(::pkl_core::Value::String(t)) => t.as_str(),
                    _ => return Err(::pkl_core::PklError::DecodeError(
                        format!("missing or invalid tag field `{}`", #tag_field)
                    )),
                };
                match __tag {
                    #(#variant_arms)*
                    _ => Err(::pkl_core::PklError::DecodeError(
                        format!("unknown tag `{}` for enum {}", __tag, stringify!(#name))
                    )),
                }
            }
        }
    };
    TokenStream::from(expanded)
}

// ── Attribute parsing ──

fn parse_field_attr(attr: &syn::Attribute, pkl_name: &mut String, has_default: &mut bool, skip: &mut bool, flatten: &mut bool) {
    let meta = match &attr.meta { Meta::List(list) => list, _ => return };
    let Ok(nested) = meta.parse_args_with(
        syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated
    ) else { return };

    for m in nested {
        match m {
            Meta::Path(path) if path.is_ident("default") => *has_default = true,
            Meta::Path(path) if path.is_ident("skip") => *skip = true,
            Meta::Path(path) if path.is_ident("flatten") => *flatten = true,
            Meta::NameValue(nv) if nv.path.is_ident("rename") => {
                if let Expr::Lit(expr_lit) = &nv.value {
                    if let Lit::Str(s) = &expr_lit.lit { *pkl_name = s.value(); }
                }
            }
            _ => {}
        }
    }
}

fn parse_enum_tag_attr(attrs: &[syn::Attribute]) -> Option<String> {
    for attr in attrs {
        if attr.path().is_ident("pkl") {
            if let Meta::List(list) = &attr.meta {
                if let Ok(nested) = list.parse_args_with(
                    syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated
                ) {
                    for m in nested {
                        if let Meta::NameValue(nv) = &m {
                            if nv.path.is_ident("tag") {
                                if let Expr::Lit(expr_lit) = &nv.value {
                                    if let Lit::Str(s) = &expr_lit.lit { return Some(s.value()); }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

fn snake_to_camel(s: &str) -> String {
    let mut result = String::new();
    let mut upper_next = false;
    for ch in s.chars() {
        if ch == '_' { upper_next = true; }
        else if upper_next { result.push(ch.to_ascii_uppercase()); upper_next = false; }
        else { result.push(ch); }
    }
    result
}

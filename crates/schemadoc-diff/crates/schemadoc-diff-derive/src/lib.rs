extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};

use quote::quote;
use syn::{
    Data, Field, Fields, GenericArgument, Path, PathArguments, Type, TypePath,
};

#[derive(Debug)]
enum FieldType {
    DiffResult,
    BoxedDiffResult,
    Other,
}

impl FieldType {
    pub fn is_other(&self) -> bool {
        matches!(self, FieldType::Other)
    }
}

fn inner_type(path: &Path) -> Option<&Type> {
    if path.leading_colon.is_some() {
        return None;
    }

    if path.segments.len() != 1 {
        return None;
    }

    let ab = match &path.segments[0].arguments {
        PathArguments::AngleBracketed(ab) => ab,
        _ => return None,
    };

    if ab.args.len() != 1 {
        return None;
    }

    match &ab.args[0] {
        GenericArgument::Type(t) => Some(t),
        _ => None,
    }
}

enum CoFieldType {
    Other,
    ContainerDiffResult,
    PrimitiveDiffResult,
}

fn is_primitive_type(path: &Path) -> bool {
    match path.get_ident() {
        None => false,
        Some(ident) => {
            matches!(
                ident.to_string().as_ref(),
                "String" | "usize" | "bool" | "f32" | "Value"
            )
        }
    }
}

fn co_get_field_type(field: &Field) -> CoFieldType {
    match &field.ty {
        Type::Path(ty) => {
            let ident = &ty.path.segments[0].ident;

            if ident == "Box" {
                match inner_type(&ty.path) {
                    Some(Type::Path(ty @ TypePath { path, .. })) => {
                        let ident = &path.segments[0].ident;
                        if ident == "DiffResult" {
                            match inner_type(&ty.path) {
                                Some(Type::Path(TypePath {
                                    path, ..
                                })) => {
                                    if is_primitive_type(path) {
                                        CoFieldType::PrimitiveDiffResult
                                    } else {
                                        CoFieldType::Other
                                    }
                                }
                                _ => unreachable!(
                                    "DiffResult must have generic type."
                                ),
                            }
                        } else {
                            CoFieldType::Other
                        }
                    }
                    None => unreachable!("Box must have generic type."),
                    _ => CoFieldType::Other,
                }
            } else if ident == "DiffResult" {
                match inner_type(&ty.path) {
                    Some(Type::Path(TypePath { path, .. })) => {
                        if is_primitive_type(path) {
                            CoFieldType::PrimitiveDiffResult
                        } else if matches!(
                            path.segments[0].ident.to_string().as_ref(),
                            "VecDiff" | "MapDiff"
                        ) {
                            CoFieldType::ContainerDiffResult
                        } else {
                            CoFieldType::Other
                        }
                    }
                    _ => unreachable!("DiffResult must have generic type."),
                }
            } else {
                CoFieldType::Other
            }
        }
        _ => CoFieldType::Other,
    }
}

#[proc_macro_derive(DiffOwnChanges)]
pub fn diff_own_changes_proc_macro(input: TokenStream) -> TokenStream {
    let syn::DeriveInput { ident, data, .. } =
        syn::parse_macro_input!(input as syn::DeriveInput);

    let data = match data {
        Data::Struct(data) => data,
        _ => panic!("Only structs are supported"),
    };

    let fields = match data.fields {
        Fields::Named(fields) => fields.named,
        _ => panic!("Only structs with names fields are supported"),
    };

    let diff_result_fields: Vec<_> = fields
        .iter()
        .map(|field| (field, co_get_field_type(field)))
        .filter(|(_, field_type)| !matches!(field_type, CoFieldType::Other))
        .collect();

    let field_idents: Vec<_> = diff_result_fields
        .iter()
        .map(|(field, field_type)| {
            let field_ident = field.ident.as_ref().unwrap();
            let field_name = field_ident.to_string();

            if matches!(field_type, CoFieldType::PrimitiveDiffResult) {
                quote! {
                    if !self.#field_ident.is_same_or_none() {
                        changes.push((#field_name.into(), (&self.#field_ident).into()))
                    }
                }
            } else {
                quote! {
                     if !self.#field_ident.is_same_or_none() {
                        changes.extend(self.#field_ident.get_own_changes())
                    }
                }
            }
        }).collect();

    let expanded = quote! {
        impl crate::diff_own_changes::DiffOwnChanges for #ident{
             fn get_own_changes(&self) -> Vec<(::std::borrow::Cow<str>, crate::diff_result_type::DiffResultType)> {
                let mut changes = Vec::new();

                 #(#field_idents)*

                changes
             }
        }
    };

    TokenStream::from(expanded)
}

fn get_field_type(field: &Field) -> FieldType {
    match &field.ty {
        Type::Path(ty) => {
            let ident = &ty.path.segments[0].ident;

            if ident == "Box" {
                match inner_type(&ty.path) {
                    Some(Type::Path(ty)) => {
                        let ident = &ty.path.segments[0].ident;
                        if ident == "DiffResult" {
                            FieldType::BoxedDiffResult
                        } else {
                            FieldType::Other
                        }
                    }
                    None => unreachable!("Box must have generic type."),
                    _ => FieldType::Other,
                }
            } else if ident == "DiffResult" {
                FieldType::DiffResult
            } else {
                FieldType::Other
            }
        }
        _ => FieldType::Other,
    }
}

#[proc_macro_derive(Empty)]
pub fn is_empty_proc_macro(input: TokenStream) -> TokenStream {
    let syn::DeriveInput { ident, data, .. } =
        syn::parse_macro_input!(input as syn::DeriveInput);

    let data = match data {
        Data::Struct(data) => data,
        _ => panic!("Only structs are supported"),
    };

    let fields = match data.fields {
        Fields::Named(fields) => fields.named,
        _ => panic!("Only structs with names fields are supported"),
    };
    let diff_result_fields: Vec<_> = fields
        .iter()
        .filter(|field| !get_field_type(field).is_other())
        .collect();

    let field_idents =
        diff_result_fields.iter().enumerate().map(|(i, field)| {
            let field_ident = field.ident.as_ref().unwrap();

            let delim = if i < diff_result_fields.len() - 1 {
                quote! { && }
            } else {
                quote! {}
            };

            quote! { self.#field_ident.is_same_or_none() #delim}
        });

    let expanded = quote! {
        impl crate::core::Empty for #ident{
             fn is_empty(&self) -> bool {
                 #(#field_idents)*
            }
        }
    };

    TokenStream::from(expanded)
}

enum FieldTypeDiff {
    Box,
    Other,
    DiffResult,
}

fn get_field_diff_type(field: &Field) -> FieldTypeDiff {
    match &field.ty {
        Type::Path(p) => {
            let ident = &p.path.segments[0].ident;

            if ident == "Box" {
                FieldTypeDiff::Box
            } else if ident == "DiffResult" {
                FieldTypeDiff::DiffResult
            } else {
                FieldTypeDiff::Other
            }
        }
        _ => FieldTypeDiff::Other,
    }
}

#[proc_macro_derive(Diff)]
pub fn diff_proc_macro(input: TokenStream) -> TokenStream {
    let syn::DeriveInput {
        ident: diff_ident,
        data,
        ..
    } = syn::parse_macro_input!(input as syn::DeriveInput);

    let data = match data {
        Data::Struct(data) => data,
        _ => panic!("Only structs are supported"),
    };

    let fields = match data.fields {
        Fields::Named(fields) => fields.named,
        _ => panic!("Only structs with names fields are supported"),
    };

    let fields: Vec<_> = fields
        .iter()
        .map(|field| (field, get_field_diff_type(field)))
        .collect();

    let removed_idents = fields
        .iter()
        .map(|(field, ty)| {
            let field_ident = field.ident.as_ref().unwrap();
            if matches!(ty, FieldTypeDiff::Box) {
                quote! { #field_ident: Box::new(self.#field_ident.diff(None, &context.removing())), }
            } else if matches!(ty, FieldTypeDiff::DiffResult) {
                quote! { #field_ident: self.#field_ident.diff(None, &context.removing()), }
            } else {
                quote! { #field_ident: self.#field_ident.clone(), }
            }
        });

    let updated_idents = fields
        .iter()
        .map(|(field, ty)| {
            let field_ident = field.ident.as_ref().unwrap();

            if matches!(ty, FieldTypeDiff::Box) {
                quote! { #field_ident: Box::new(self.#field_ident.diff(Option::from(&*value.#field_ident), context)), }
            } else if matches!(ty, FieldTypeDiff::DiffResult) {
                quote! { #field_ident: self.#field_ident.diff(Option::from(&value.#field_ident), context), }
            } else {
                quote! { #field_ident: self.#field_ident.clone(), }
            }
        });

    let diff_ident_name = diff_ident.to_string();
    let ident =
        Ident::new(&diff_ident_name.replace("Diff", ""), Span::call_site());

    let expanded = quote! {
        impl Diff<crate::schema::#ident, #diff_ident, crate::context::HttpSchemaDiffContext> for crate::schema::#ident {
            fn diff(
                &self,
                new: Option<&crate::schema::#ident>,
                context: &crate::context::HttpSchemaDiffContext,
            ) -> DiffResult<#diff_ident> {
                let diff = match new {
                    None => DiffResult::Removed(#diff_ident {
                        #(#removed_idents)*
                    }),
                    Some(value) => {
                        let diff = #diff_ident {
                            #(#updated_idents)*
                        };

                        if diff.is_empty() {
                            DiffResult::Same(diff)
                        } else {
                            DiffResult::Updated(diff, None)
                        }
                    }
                };
                DiffResult::new(diff, context)
            }
        }
    };

    TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {}

use crate::struct_kind::StructKind;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::DeriveInput;
use syn::*;

pub(crate) struct InjectableStruct<'a> {
    ident: &'a Ident,
    generics: &'a Generics,
    kind: StructKind<'a>,
}

impl<'a> InjectableStruct<'a> {
    pub fn new(input: &'a DeriveInput) -> Self {
        let ident = &input.ident;
        let generics = &input.generics;

        let kind = match &input.data {
            syn::Data::Struct(data_struct) => match &data_struct.fields {
                syn::Fields::Named(fields) => StructKind::Named(fields),
                syn::Fields::Unnamed(fields) => StructKind::Unnamed(fields),
                syn::Fields::Unit => StructKind::Unit,
            },
            _ => panic!("Injectable can only be derived on structs."),
        };

        InjectableStruct {
            ident,
            generics,
            kind,
        }
    }

    fn fields(&self) -> Vec<&syn::Field> {
        match self.kind {
            StructKind::Named(fields) => fields.named.iter().collect(),
            StructKind::Unnamed(fields) => fields.unnamed.iter().collect(),
            StructKind::Unit => vec![],
        }
    }

    fn parse_dependencies(
        &self,
    ) -> (
        Vec<&Type>,       // dep_types
        Vec<TokenStream>, // dep_tokens
        Vec<TokenStream>, // factory_tokens (named use ident: expr)
        Vec<TokenStream>, // factory_exprs  (unnamed use expr only)
    ) {
        let mut dep_types = Vec::new();
        let mut dep_tokens = Vec::new();
        let mut factory_tokens = Vec::new();
        let mut factory_exprs = Vec::new();

        for field in self.fields() {
            if let Some(attr) = field.attrs.iter().find(|a| a.path().is_ident("inject")) {
                let expr: Expr = match attr.parse_args() {
                    Ok(ex) => ex,
                    Err(_) => {
                        let tokens = attr.meta.require_list()
                            .expect("expected #[inject(...)]")
                            .tokens.clone();

                        syn::parse2(tokens)
                            .expect("expected valid closure like #[inject(|| expr)]")
                    }
                };

                let mut expr_ref = &expr;
                while let Expr::Paren(paren) = expr_ref {
                    expr_ref = &*paren.expr;
                }

                let factory_expr = match expr_ref {
                    Expr::Closure(c) => quote! { (#c)() },
                    _ => quote! { ::core::default::Default::default() },
                };


                // Save raw expression for tuple struct constructor
                factory_exprs.push(factory_expr.clone());

                // Get field name (or derive if tuple)
                let ident = match self.kind {
                    StructKind::Named(_) => field.ident.as_ref().unwrap().clone(),
                    StructKind::Unnamed(_) => {
                        if let Type::Path(path) = &field.ty {
                            let ty_ident = &path.path.segments.last().unwrap().ident;
                            format_ident!("{}", self.to_snake_case(&ty_ident.to_string()))
                        } else {
                            panic!("Unsupported type for unnamed inject field");
                        }
                    }
                    StructKind::Unit => continue,
                };

                // Named: ident: expr → `b: (|| 4)()`
                factory_tokens.push(quote! { #ident: #factory_expr });
            } else {
                // Dependency case
                dep_types.push(&field.ty);
                dep_tokens.push(if let StructKind::Named(_) = self.kind {
                    let ident = field.ident.as_ref().unwrap();
                    quote! { #ident }
                } else if let Type::Path(path) = &field.ty {
                    let ty_ident = &path.path.segments.last().unwrap().ident;
                    let ident = format_ident!("{}", self.to_snake_case(&ty_ident.to_string()));
                    quote! { #ident }
                } else {
                    panic!("Unsupported type format for unnamed DI");
                });
            }
        }

        (dep_types, dep_tokens, factory_tokens, factory_exprs)
    }

    pub fn into_token_stream(&self) -> TokenStream {
        let ident = self.ident;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

        let (dep_types, dep_tokens, factory_tokens, factory_exprs) = self.parse_dependencies();

        let inject_params = if dep_tokens.is_empty() {
            quote! { _: Self::Deps }   // correctly ignore dependency list
        } else {
            quote! { ( #(#dep_tokens),* ): Self::Deps }
        };

        match self.kind {
            StructKind::Named(_) => {
                let mut tokens = Vec::new();
                tokens.extend(dep_tokens.iter().cloned());
                tokens.extend(factory_tokens.iter().cloned());

                quote! {
                    impl #impl_generics Injectable for #ident #ty_generics #where_clause {
                        type Deps = ( #(#dep_types),* );
                        fn inject(#inject_params) -> Self {
                            Self { #(#tokens),* }
                        }
                    }
                }
            }

            StructKind::Unnamed(_) => {
                let mut tokens = Vec::new();
                tokens.extend(dep_tokens.iter().cloned());
                tokens.extend(factory_exprs.iter().cloned());

                quote! {
                    impl #impl_generics Injectable for #ident #ty_generics #where_clause {
                        type Deps = ( #(#dep_types),* );
                        fn inject(#inject_params) -> Self {
                            Self( #(#tokens),* )
                        }
                    }
                }
            }

            StructKind::Unit => quote! {
        impl #impl_generics Injectable for #ident #ty_generics #where_clause {
            type Deps = ();
            fn inject(_: Self::Deps) -> Self {
                Self
            }
        }
    }
        }

    }
    fn to_snake_case(&self, s: &str) -> String {
        let mut result = String::new();

        for (i, ch) in s.chars().enumerate() {
            if ch.is_uppercase() {
                if i != 0 {
                    result.push('_');
                }
                result.push(ch.to_ascii_lowercase());
            } else {
                result.push(ch);
            }
        }
        result
    }
}



#[cfg(test)]
mod test {
    use super::*; // Import everything from the parsing module
    use syn::{parse_quote, DeriveInput};

    #[test]
    fn struct_named_simple() {
        let input: DeriveInput = parse_quote! {
            struct A {
                x: i32,
            }
        };

        let result = InjectableStruct::new(&input);

        assert_eq!(result.ident.to_string(), "A");
        assert!(result.generics.params.is_empty(), "Should have no generics");

        match result.kind {
            StructKind::Named(_) => {}
            _ => panic!("Expected Named struct"),
        }
    }

    #[test]
    fn struct_tuple() {
        let input: DeriveInput = parse_quote! {
            struct B(i32, String);
        };

        let result = InjectableStruct::new(&input);

        match result.kind {
            StructKind::Unnamed(_) => {}
            _ => panic!("Expected Tuple struct"),
        }
    }

    #[test]
    fn struct_unit() {
        let input: DeriveInput = parse_quote! {
            struct C;
        };

        let result = InjectableStruct::new(&input);

        match result.kind {
            StructKind::Unit => {}
            _ => panic!("Expected Unit struct"),
        }
    }

    #[test]
    fn struct_with_generics() {
        let input: DeriveInput = parse_quote! {
            struct D<T> {
                v: T,
            }
        };

        let result = InjectableStruct::new(&input);

        assert!(!result.generics.params.is_empty(), "Should detect generic parameters");
        assert_eq!(result.ident.to_string(), "D");
    }

    #[test]
    #[should_panic(expected = "Injectable can only be derived on structs.")] // örnek hata mesajı
    fn enum_not_supported() {
        let input: DeriveInput = parse_quote! {
            enum E { V }
        };

        InjectableStruct::new(&input);
    }



    #[test]
    fn generated_impl_matches_expected() {
        // Arrange
        let input: syn::DeriveInput = parse_quote! {
            struct TestService {
                a: i32,
            }
        };

        // Act
        let tokens =  &InjectableStruct::new(&input).into_token_stream();


        // Assert
        assert!(
            tokens.to_string().contains("impl Injectable for TestService"),
            "Generated code must contain trait implementation."
        );


        assert!(
            tokens.to_string().contains("type Deps = (i32)"),
            "Dependency tuple must be generated"
        );
        assert!(
            tokens.to_string().contains("fn inject"),
            "Inject function must exist"
        );
        assert!(
            tokens.to_string().contains("{ a }"),
            "Field initialization should match"
        );
    }




    #[test]
    fn generated_impl_for_generic_struct() {
        let input: syn::DeriveInput = parse_quote! {
        struct GenericService<T> {
            value: T,
        }
    };

        let tokens =  &InjectableStruct::new(&input.into()).into_token_stream();
        let code = tokens.to_string();



        // Struct adı doğru mu?
        assert!(
            code.contains("impl < T > Injectable for GenericService < T >"),
            "Generic impl block must include generics"
        );

        // Deps tuple generics içeriyor mu?
        assert!(
            code.contains("type Deps = (T)"),
            "Generic dependency tuple must be T"
        );

        // inject metodu doğru mu?
        assert!(
            code.contains("fn inject"),
            "Inject method missing"
        );

        assert!(
            code.contains("{ value }"),
            "Field initialization incorrect"
        );
    }
}
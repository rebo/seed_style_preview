extern crate proc_macro;
use self::proc_macro::TokenStream;
use heck::KebabCase;
use heck::SnakeCase;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, DeriveInput, Expr, ExprArray};
use syn::{Lit, Meta, MetaNameValue};

// Derive Proc Macro to generate extension traits for Style
// add helpers for variants of properties that have no arguments
// excluding `Initial` and `Initial`.
//
// For instance if the enum is
// #[short_prop = "p"]
// CssPadding {
//     Auto,
//     Length(ExactLength),
//     Initial,
//     Inherit,
//  }
/// then `padding_auto()` will be generated on Style.
//  if short_prop attribute is set, then that will be set too
//  i.e. `p_auto()`
#[proc_macro_derive(CssStyleMacro, attributes(short_prop))]
pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let mut short_prop: Option<String> = None;

    // Iterate over the struct's #[...] attributes
    for option in input.attrs.into_iter() {
        let option = option.parse_meta().unwrap();
        match option {
            // Match '#[ident = lit]' attributes. Match guard makes it '#[short_prop = lit]'
            Meta::NameValue(MetaNameValue {
                ref path, ref lit, ..
            }) if path.is_ident("short_prop") => {
                if let Lit::Str(lit) = lit {
                    short_prop = Some(lit.value());
                }
            }
            _ => {}
        }
    }

    let short_prop = short_prop.clone();

    let css_type_name = input.ident.clone();
    let variant_ident = format_ident!(
        "{}",
        css_type_name
            .to_string()
            .trim_start_matches("Css")
            .to_string()
    );
    let trait_name_ident = format_ident!("{}Trait", input.ident);
    let snake_case_type = variant_ident.to_string().to_snake_case();
    let snake_case_type_ident = format_ident!("{}", snake_case_type);

    let (base_defn_quote, base_impl_quote) = if let Some(short_prop) = short_prop.clone() {
        let short_prop_ident = format_ident!("{}", short_prop);
        (
            quote!(
                #[track_caller]
                fn #snake_case_type_ident<T>(&self, val:T) -> Style  where T: UpdateStyle<#css_type_name> ;

                 #[track_caller]
                 fn #short_prop_ident<T>(&self, val:T) -> Style  where T: UpdateStyle<#css_type_name>;
            ),
            quote!(

            #[track_caller]
            fn #snake_case_type_ident<T>(&self, val:T) -> Style  where T: UpdateStyle<#css_type_name> {
                let mut new_style = self.clone();
                new_style.updated_at.push(format!("{}", Location::caller()));
                val.update_style(&mut new_style);
                new_style

             }

             #[track_caller]
             fn #short_prop_ident<T>(&self, val:T) -> Style  where T: UpdateStyle<#css_type_name> {
                let mut new_style = self.clone();
                new_style.updated_at.push(format!("{}", Location::caller()));
                 val.update_style(&mut new_style);
                 new_style
              }
            ),
        )
    } else {
        (
            quote!(

            #[track_caller]
            fn #snake_case_type_ident<T>(&self, val:T) -> Style  where T: UpdateStyle<#css_type_name>;

            ),
            quote!(

            #[track_caller]
            fn #snake_case_type_ident<T>(&self, val:T) -> Style  where T: UpdateStyle<#css_type_name> {
                let mut new_style = self.clone();
                new_style.updated_at.push(format!("{}", Location::caller()));
                val.update_style(&mut  new_style);
                new_style
             }

            ),
        )
    };

    // let inner_quote = quote! {
    //     fn update_style(self, style: &Style)-> Style {
    //         with_themes( |borrowed_themes| {
    //             let new_style = style.clone();
    //             for (i, style_val) in &mut self.iter().cloned().map(|v| v.into()).enumerate() {
    //                 if let Some(breakpoint) = borrowed_themes.iter().find_map( |theme| theme.media_bp_scale.get(i) ){
    //                     new_style.updated_at.push(format!("{}", Location::caller()));
    //                     let rules = new_style.media_rules.entry(breakpoint.0.clone()).or_insert(vec![]);
    //                     rules.push(Rule{value:Box::new(style_val.clone())})
    //                 }
    //             }
    //             new_style
    //         })
    //     }
    // };

    let update_style_quote = quote! {
        impl CssValueTrait for #css_type_name{}

    // impl UpdateStyle<#css_type_name> for #css_type_name {
    //     fn update_style(self, style: &mut Style)-> Style{
    //         let val : #css_type_name = self.into();



    //         let new_style =  style.clone();
    //         new_style.updated_at.push(format!("{}", Location::caller()));
    //         new_style.add_rule(Box::new(val));
    //         new_style
    //     }
    // }

    // impl UpdateStyle<#css_type_name> for &str {
    //     fn update_style(self, style: &mut Style)-> Style{
    //         let new_style =  style.clone();
    //         let val : #css_type_name = self.into();
    //         new_style.updated_at.push(format!("{}", Location::caller()));
    //         new_style.add_rule(Box::new(val));
    //         new_style
    //     }
    // }



    impl From<&str> for #css_type_name where {
        fn from(v: &str) -> Self {
            #css_type_name::StringValue(v.to_string())
        }
    }

    // impl <R>UpdateStyle<#css_type_name> for &Vec<R> where R:Into<#css_type_name> + Clone{#inner_quote }
    // impl <R>UpdateStyle<#css_type_name> for &[R] where R:Into<#css_type_name> + Clone{#inner_quote }
    // impl <R>UpdateStyle<#css_type_name> for &[R;1] where R:Into<#css_type_name> + Clone{
    //     fn update_style(self, style: &Style)-> Style {
    //         self.as_ref().update_style(style)
    //     }
    // }

    // impl <R>UpdateStyle<#css_type_name> for &[R;2] where R:Into<#css_type_name> + Clone{
    //     fn update_style(self, style: &Style)-> Style {
    //         self.as_ref().update_style(style)
    //     }
    // }
    // impl <R>UpdateStyle<#css_type_name> for &[R;3] where R:Into<#css_type_name> + Clone{
    //     fn update_style(self, style: &Style)-> Style {
    //         self.as_ref().update_style(style)
    //     }
    // }
    // impl <R>UpdateStyle<#css_type_name> for &[R;4] where R:Into<#css_type_name> + Clone{
    //     fn update_style(self, style: &Style)-> Style {
    //         self.as_ref().update_style(style)
    //     }
    // }

    // impl <R>UpdateStyle<#css_type_name> for &[R;5] where R:Into<#css_type_name> + Clone{
    //     fn update_style(self, style: &Style)-> Style {
    //         self.as_ref().update_style(style)
    //     }
    // }

    // impl <R>UpdateStyle<#css_type_name> for &[R;6] where R:Into<#css_type_name> + Clone{
    //     fn update_style(self, style: &Style)-> Style {
    //         self.as_ref().update_style(style)
    //     }
    // }

    // impl <R>UpdateStyle<#css_type_name> for &[R;7] where R:Into<#css_type_name> + Clone{
    //     fn update_style(self, style: &Style)-> Style {
    //         self.as_ref().update_style(style)
    //     }
    // }
    // impl <R>UpdateStyle<#css_type_name> for &[R;8] where R:Into<#css_type_name> + Clone{
    //     fn update_style(self, style: &Style)-> Style {
    //         self.as_ref().update_style(style)
    //     }
    // }

    // impl <R>UpdateStyle<#css_type_name> for &[R;9] where R:Into<#css_type_name> + Clone{
    //     fn update_style(self, style: &Style)-> Style {
    //         self.as_ref().update_style(style)
    //     }
    // }

    // impl <R>UpdateStyle<#css_type_name> for &[R;10] where R:Into<#css_type_name> + Clone{
    //     fn update_style(self, style: &Style)-> Style {
    //         self.as_ref().update_style(style)
    //     }
    // }
    };

    let mut create_extension_trait = false;

    if let syn::Data::Enum(data_enum) = input.data {
        let count = data_enum
            .variants
            .iter()
            .filter(|v| v.fields == syn::Fields::Unit)
            .filter(|v| v.ident.clone().to_string() != "Inherit".to_string())
            .filter(|v| v.ident.clone().to_string() != "Initial".to_string())
            .count();
        if count > 0 {
            create_extension_trait = true;
        }

        let func_impls_defn = data_enum
            .variants
            .iter()
            .filter(|v| v.fields == syn::Fields::Unit)
            .filter(|v| v.ident.clone().to_string() != "Inherit".to_string())
            .filter(|v| v.ident.clone().to_string() != "Initial".to_string())
            .map(|v| {
                let snake_case_variant = v.ident.clone().to_string().to_snake_case();
                let value_variant_name = format_ident!(
                    "{}",
                    css_type_name
                        .clone()
                        .to_string()
                        .trim_start_matches("Css")
                        .to_string()
                );
                let snake_case_type = value_variant_name.clone().to_string().to_snake_case();

                let f_small_name = format_ident!("{}_{}", snake_case_type, snake_case_variant);
                let f_small_name_ident = format_ident!("{}", f_small_name);

                if let Some(short_prop) = short_prop.clone() {
                    let short_prop_ident = format_ident!("{}_{}", short_prop, snake_case_variant);
                    quote! {
                        fn #f_small_name_ident(&self) -> Style;
                        fn #short_prop_ident(&self) -> Style;

                    }
                } else {
                    quote! {
                        fn #f_small_name_ident(&self) -> Style;
                    }
                }
            });

        let trait_definition = quote! {
            pub trait #trait_name_ident {
                #(
                #func_impls_defn
                )*

                #base_defn_quote



            }
        };

        let func_impls = data_enum
            .variants
            .iter()
            .filter(|v| v.fields == syn::Fields::Unit)
            .filter(|v| v.ident.clone().to_string() != "Inherit".to_string())
            .filter(|v| v.ident.clone().to_string() != "Initial".to_string())
            .map(|v| {
                let f_big_name = v.ident.clone();
                let snake_case_variant = v.ident.clone().to_string().to_snake_case();
                // let snake_case_type = value_variant_name.clone().to_string().to_snake_case();

                let f_small_name = format_ident!("{}_{}", snake_case_type, snake_case_variant);
                let f_small_name_ident = format_ident!("{}", f_small_name);
                if let Some(short_prop) = short_prop.clone() {
                    let short_prop_ident = format_ident!("{}_{}", short_prop, snake_case_variant);
                    quote! {
                        #[track_caller]
                        fn #short_prop_ident(&self) -> Style {
                            let mut new_style = self.clone();

                            new_style.updated_at.push(format!("{}", Location::caller()));
                            new_style.add_rule(Box::new(#css_type_name :: #f_big_name));
                            new_style
                        }
                        #[track_caller]
                            fn #f_small_name_ident(&self) -> Style {
                            let mut new_style = self.clone();

                            new_style.updated_at.push(format!("{}", Location::caller()));
                            new_style.add_rule(Box::new(#css_type_name :: #f_big_name));
                            new_style
                        }
                    }
                } else {
                    quote! {
                        #[track_caller]
                        fn #f_small_name_ident(&self) -> Style {
                            let mut new_style = self.clone();
                            new_style.updated_at.push(format!("{}", Location::caller()));
                            new_style.add_rule(Box::new(#css_type_name :: #f_big_name));
                            new_style
                        }

                    }
                }
            });

        let trait_impl = quote! {

            impl #trait_name_ident for Style {
                #(#func_impls)*
                #base_impl_quote

            }
        };

        if create_extension_trait {
            let exp = quote! {
                #update_style_quote
                #trait_definition
                #trait_impl
            };

            exp.into()
        } else {
            let exp = quote! {
                    pub trait #trait_name_ident {
                      #base_defn_quote
                    }


                impl #trait_name_ident for Style {
                    #base_impl_quote
                }
                    #update_style_quote
            };
            exp.into()
        }
    } else {
        let exp = quote! {
            pub trait #trait_name_ident {
                #base_defn_quote
            }


        impl #trait_name_ident for Style {
            #base_impl_quote
        }
        #update_style_quote
        };

        exp.into()
    }
}

// // Prop Macro to generate all short property methods on Style.
// // Will eventually be moved to the derive macro above.
// struct GenerateShortFNames {
//     properties: ExprArray,
// }

// impl Parse for GenerateShortFNames {
//     fn parse(input: ParseStream) -> Result<Self> {
//         let properties: ExprArray = input.parse()?;

//         Ok(GenerateShortFNames { properties })
//     }
// }

// #[proc_macro]
// pub fn generate_short_f_names(input: TokenStream) -> TokenStream {
//     let GenerateShortFNames { properties } = parse_macro_input!(input as GenerateShortFNames);

//     let mut exp = quote! {};
//     for property in properties.elems.iter() {
//         if let Expr::Tuple(ref tuple) = property {
//             let mut iter = tuple.elems.iter();
//             let theme_id = iter.next().unwrap().clone();
//             let generic_type_name = iter.next().unwrap().clone();

//             if let (Expr::Lit(ref theme_id), Expr::Lit(ref generic_type_name)) =
//                 (theme_id, generic_type_name)
//             {
//                 if let (syn::Lit::Str(ref short_name), syn::Lit::Str(ref long_name)) =
//                     (theme_id.lit.clone(), generic_type_name.lit.clone())
//                 {
//                     let short_name_ident = format_ident!("{}", short_name.value());
//                     let long_name_ident = format_ident!("Css{}", long_name.value());

//                     let variant_name = format_ident!("{}", long_name.value());

//                     let expanded = quote! {

//                        #[track_caller]
//                        pub fn #short_name_ident<T>(&self, val:T) -> Style  where T: UpdateStyle<#long_name_ident> {
//                            val.update_style(self)
//                        }

//                     };

//                     // println!("{}",expanded);
//                     exp = quote! {
//                         #exp
//                         #expanded
//                     };

//                     // println!("{}",TokenStream::from(exp));
//                 }
//             }
//         }
//     }

//     // let elem_ident = Ident::from(elem);

//     // let exp = quote!{};

//     exp.into()
// }

// impl From<&str> for #type_name where {
//         fn from(v: &str) -> Self {
//             #type_name::StringValue(v.to_string())
//         }
//     }

// Proc macro to impl from<&str> for Css values
// allows StringValue variant to be created.
struct GenerateFromStr {
    properties: ExprArray,
}

impl Parse for GenerateFromStr {
    fn parse(input: ParseStream) -> Result<Self> {
        let properties: ExprArray = input.parse()?;

        Ok(GenerateFromStr { properties })
    }
}

#[proc_macro]
pub fn generate_from_strs(input: TokenStream) -> TokenStream {
    let GenerateFromStr { properties } = parse_macro_input!(input as GenerateFromStr);

    let mut exp = quote! {};
    for property in properties.elems.iter() {
        if let Expr::Lit(ref property) = property {
            if let syn::Lit::Str(ref property) = property.lit {
                let type_name = format_ident!("Css{}", property.value());

                let expanded = quote! {

                impl From<&str> for #type_name where {
                        fn from(v: &str) -> Self {
                            #type_name::StringValue(v.to_string())
                        }
                    }

                };
                // println!("{}",expanded);
                exp = quote! {
                    #exp
                    #expanded
                };
            }
        }
    }

    // let exp = quote!{};
    TokenStream::from(exp)
}

// #[track_caller]
// pub fn hover(&self) -> Style {
//     let new_style = self.clone();
//     new_style.updated_at.push(format!("{}", Location::caller()));
//     new_style.pseudo = Pseudo::Hover;
//     new_style
// }

// psuedo selectors:
//
// Proc macro to generate psuedo methods for Style
struct CreatePseudos {
    properties: ExprArray,
}

impl Parse for CreatePseudos {
    fn parse(input: ParseStream) -> Result<Self> {
        let properties: ExprArray = input.parse()?;

        Ok(CreatePseudos { properties })
    }
}

#[proc_macro]
pub fn create_pseudos(input: TokenStream) -> TokenStream {
    let CreatePseudos { properties } = parse_macro_input!(input as CreatePseudos);

    let mut exp = quote! {};
    for property in properties.elems.iter() {
        if let Expr::Lit(ref property) = property {
            if let syn::Lit::Str(ref property) = property.lit {
                let pseudoname = format_ident!("{}", property.value());

                let fname = format_ident!("{}", property.value().to_snake_case());

                let expanded = quote! {



                #[track_caller]
                pub fn #fname(&self) -> Style {
                    let mut new_style = self.clone();
                    new_style.updated_at.push(format!("{}", Location::caller()));
                    new_style.pseudo = Pseudo::#pseudoname;
                    new_style
                }

                };
                // println!("{}",expanded);
                exp = quote! {
                    #exp
                    #expanded
                };
            }
        }

        // let elem_ident = Ident::from(elem);
    }

    // let exp = quote!{};
    TokenStream::from(exp)
}

//
// impl <T> From<T> for CssWidth where T:SizeTheme + 'static{
//     fn from(v: T) -> Self {
//         let theme = get_theme();
//         let borrowed_theme = theme.borrow();
//         borrowed_theme.get::<T,CssSize>(v).unwrap().into()
//     }
// }

// impl <T>From<(T,CssWidth)> for CssWidth where T: SizeTheme + 'static {
//     fn from(v:(T,CssWidth)) -> Self {
//         let theme = get_theme();
//         let borrowed_theme = theme.borrow();
//         if let Some(theme_value) = borrowed_theme.get::<T,CssSpace>(v.0){
//             theme_value.clone().into()
//         } else {
//             v.1
//         }
//     }
// }
////
//
// Proc macro to generate Css Values from theme variants
//
// [ (ThemeId, GenericTypeName, SpecificTypeName )]
struct CreateFromTraits {
    properties: ExprArray,
}

impl Parse for CreateFromTraits {
    fn parse(input: ParseStream) -> Result<Self> {
        let properties: ExprArray = input.parse()?;

        Ok(CreateFromTraits { properties })
    }
}

#[proc_macro]
pub fn generate_froms(input: TokenStream) -> self::proc_macro::TokenStream {
    let CreateFromTraits { properties } = parse_macro_input!(input as CreateFromTraits);

    let mut exp = quote! {};
    for property in properties.elems.iter() {
        if let Expr::Tuple(ref tuple) = property {
            let mut iter = tuple.elems.iter();
            let theme_id = iter.next().unwrap().clone();
            let generic_type_name = iter.next().unwrap().clone();
            let specific_type_name = iter.next().unwrap().clone();
            let theme_scale = iter.next().unwrap().clone();

            if let (
                Expr::Lit(ref theme_id),
                Expr::Lit(ref generic_type_name),
                Expr::Lit(ref specific_type_name),
                Expr::Lit(ref theme_scale),
            ) = (theme_id, generic_type_name, specific_type_name, theme_scale)
            {
                if let (
                    syn::Lit::Str(ref theme_ids),
                    syn::Lit::Str(ref generic_type_names),
                    syn::Lit::Str(ref specific_type_names),
                    syn::Lit::Str(ref theme_scale),
                ) = (
                    theme_id.lit.clone(),
                    generic_type_name.lit.clone(),
                    specific_type_name.lit.clone(),
                    theme_scale.lit.clone(),
                ) {
                    let themeid_ident = format_ident!("{}", theme_ids.value());
                    let generic_ident = format_ident!("{}", generic_type_names.value());
                    let specific_ident = format_ident!("{}", specific_type_names.value());

                    let theme_scale_ident = format_ident!("{}", theme_scale.value());
                    let theme_scale_string = theme_scale.value();
                    let expanded = quote! {


                            impl <T> From<T> for #specific_ident where T:#themeid_ident + 'static{
                                fn from(v: T) -> Self {
                                    with_themes( |borrowed_themes| {
                                    borrowed_themes.iter().find_map( |theme| theme.get::<T,#generic_ident>(v.clone())).unwrap().into()
                                    })
                                }
                            }

                            impl <T>From<(T,#generic_ident)> for #specific_ident where T: #themeid_ident + 'static {
                                fn from(v:(T,#generic_ident)) -> Self {
                                    with_themes( |borrowed_themes| {

                                    if let Some(theme_value) = borrowed_themes.iter().find_map( |theme| theme.get::<T,#generic_ident>(v.0.clone())){
                                        theme_value.clone().into()
                                    } else {
                                        v.1.clone().into()
                                    }
                                    })
                                }
                            }

                            impl<Th> UpdateStyle<#specific_ident> for Th
                                where Th: #themeid_ident + 'static {
                                fn update_style(self, style: &mut Style) {
                                    let val : #specific_ident = self.into();

                                    style.add_rule(Box::new(val));

                                }
                            }


                            impl <Th>UpdateStyle<#specific_ident> for (Th, #generic_ident)
                                    where Th: #themeid_ident + 'static {
                                    fn update_style(self, style: &mut Style) {
                                        let val : #specific_ident = self.into();

                                        style.add_rule(Box::new(val));

                                    }
                                }

                            impl UpdateStyle<#specific_ident> for usize {
                                fn update_style(self, style: &mut Style){
                                     *style =
                                     with_themes( |borrowed_themes| {

                                        let theme_value : #specific_ident =
                                            if let Some(theme_value) = borrowed_themes.iter().find_map( |theme| theme.#theme_scale_ident.get(self) ){
                                                theme_value.clone().into()

                                            } else {

                                                panic!("Theme scale does not exist {}", #theme_scale_string )

                                            };

                                        let mut new_style = style.clone();
                                        new_style.add_rule(Box::new(theme_value));
                                        new_style
                                     });
                                }
                        }
                        impl UpdateStyle<#specific_ident> for &[usize; 1]
                        where
                        #specific_ident: 'static + Clone + CssValueTrait,
                         {
                            fn update_style(self, style: &mut Style) {
                                let ref_self : &[usize] = self.as_ref();
                                <&[usize] as UpdateStyle<#specific_ident>>::update_style(ref_self, style);
                            }
                        }

                        impl UpdateStyle<#specific_ident> for &[usize; 2]
                        where
                        #specific_ident: 'static + Clone + CssValueTrait,
                         {
                            fn update_style(self, style: &mut Style) {
                                let ref_self : &[usize] = self.as_ref();
                                <&[usize] as UpdateStyle<#specific_ident>>::update_style(ref_self,style);
                            }
                        }

                        impl UpdateStyle<#specific_ident> for &[usize; 3]
                        where
                        #specific_ident: 'static + Clone + CssValueTrait,
                         {
                            fn update_style(self, style: &mut Style) {
                                let ref_self : &[usize] = self.as_ref();
                                <&[usize] as UpdateStyle<#specific_ident>>::update_style(ref_self,style);
                            }
                        }


                        impl UpdateStyle<#specific_ident> for &[usize; 4]
                        where
                        #specific_ident: 'static + Clone + CssValueTrait,
                         {
                            fn update_style(self, style: &mut Style) {
                                let ref_self : &[usize] = self.as_ref();
                                <&[usize] as UpdateStyle<#specific_ident>>::update_style(ref_self,style);
                            }
                        }

                        impl UpdateStyle<#specific_ident> for &[usize; 5]
                        where
                        #specific_ident: 'static + Clone + CssValueTrait,
                         {
                            fn update_style(self, style: &mut Style) {
                                let ref_self : &[usize] = self.as_ref();
                                <&[usize] as UpdateStyle<#specific_ident>>::update_style(ref_self,style);
                            }
                        }

                        impl UpdateStyle<#specific_ident> for &[usize; 6]
                        where
                        #specific_ident: 'static + Clone + CssValueTrait,
                         {
                            fn update_style(self, style: &mut Style) {
                                let ref_self : &[usize] = self.as_ref();
                                <&[usize] as UpdateStyle<#specific_ident>>::update_style(ref_self,style);
                            }
                        }

                        impl UpdateStyle<#specific_ident> for &[usize; 7]
                        where
                        #specific_ident: 'static + Clone + CssValueTrait,
                         {
                            fn update_style(self, style: &mut Style) {
                                let ref_self : &[usize] = self.as_ref();
                                <&[usize] as UpdateStyle<#specific_ident>>::update_style(ref_self,style);
                            }
                        }


                        impl UpdateStyle<#specific_ident> for &[usize; 8]
                        where
                        #specific_ident: 'static + Clone + CssValueTrait,
                         {
                            fn update_style(self, style: &mut Style) {
                                let ref_self : &[usize] = self.as_ref();
                                <&[usize] as UpdateStyle<#specific_ident>>::update_style(ref_self,style);
                            }
                        }

                        impl UpdateStyle<#specific_ident> for &[usize; 9]
                        where
                        #specific_ident: 'static + Clone + CssValueTrait,
                         {
                            fn update_style(self, style: &mut Style) {
                                let ref_self : &[usize] = self.as_ref();
                                <&[usize] as UpdateStyle<#specific_ident>>::update_style(ref_self,style);
                            }
                        }


                        impl UpdateStyle<#specific_ident> for &[usize; 10]
                        where
                        #specific_ident: 'static + Clone + CssValueTrait,
                         {
                            fn update_style(self, style: &mut Style) {
                                let ref_self : &[usize] = self.as_ref();
                                <&[usize] as UpdateStyle<#specific_ident>>::update_style(ref_self,style);
                            }
                        }





                        impl UpdateStyle<#specific_ident> for &[usize] where
                        #specific_ident: 'static + Clone + CssValueTrait,{
                            fn update_style(self, style: &mut Style){

                                *style = with_themes( |borrowed_themes| {


                                    let mut  new_style = style.clone();
                                    if let Some(bp_scale) = &borrowed_themes.iter().find_map(|theme| if !theme.media_bp_scale.is_empty() { Some(theme.media_bp_scale.clone())} else { None }) {


                                        let mut old_style = None;

                                        for (style_idx, bp) in bp_scale.iter().enumerate(){
                                            if let Some(theme_idx) = self.get(style_idx){
                                            if let Some(generic_value )= borrowed_themes
                                                .iter()
                                                .find_map( |theme| theme.#theme_scale_ident.get(*theme_idx)).cloned(){

                                                    let specific_value : #specific_ident = generic_value.into();

                                                    let rules = new_style
                                                                    .media_rules
                                                                    .entry(bp.clone().0)
                                                                    .or_insert(vec![]);
                                                    rules.push(Rule {
                                                        value: Box::new(specific_value.clone()),
                                                    });

                                                    old_style = Some(specific_value);
                                                }else {
                                                    panic!("No theme scale for that index!")
                                                }

                                            } else {
                                                    let rules = new_style
                                                        .media_rules
                                                        .entry(bp.clone().0)
                                                        .or_insert(vec![]);
                                                        rules.push(Rule {
                                                            value: Box::new(old_style.clone().unwrap()),
                                                        });

                                                }


                                        }
                                    } else {
                                        panic!("No breakpoints have been defined!")

                                    }



                                    new_style
                                });

                            }
                        }

                    };

                    let ident_from_generic = if generic_ident != specific_ident {
                        quote!(
                            impl UpdateStyle<#specific_ident> for #generic_ident {
                            fn update_style(self, style: &mut Style) {
                                let val : #specific_ident = self.into();
                                style.add_rule(Box::new(val));
                            }
                        }
                        )
                    } else {
                        quote!()
                    };

                    // println!("{}", expanded);
                    exp = quote! {
                        #ident_from_generic
                        #exp
                        #expanded
                    };
                }
            }
        }
    }

    exp.into()
}

struct CreateEnums {
    properties: ExprArray,
}

impl Parse for CreateEnums {
    fn parse(input: ParseStream) -> Result<Self> {
        let properties: ExprArray = input.parse()?;

        Ok(CreateEnums { properties })
    }
}

#[proc_macro]
pub fn create_enums(input: TokenStream) -> TokenStream {
    let CreateEnums { properties } = parse_macro_input!(input as CreateEnums);

    let mut exp = quote! {};
    for property in properties.elems.iter() {
        if let Expr::Lit(ref property) = property {
            if let syn::Lit::Str(ref property) = property.lit {
                let type_name = format_ident!("Css{}", property.value());

                let css_name = format!("{}: {{}};", property.value().to_kebab_case());

                let expanded = quote! {

                    #[derive(Display,Clone,Debug,CssStyleMacro)]
                    #[display(fmt = #css_name)]
                    pub enum #type_name {
                        #[display(fmt = "inherit")]
                        Inherit,
                        StringValue(String),
                    }


                };
                // println!("{}",expanded);
                exp = quote! {
                    #exp
                    #expanded
                };
            }
        }
    }

    TokenStream::from(exp)
}

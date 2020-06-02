extern crate darling;
extern crate proc_macro;
use self::proc_macro::TokenStream;
use heck::CamelCase;
use heck::KebabCase;
use heck::SnakeCase;

use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, DeriveInput, Expr, ExprArray};
use syn::{FnArg, ItemFn, Pat};
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
#[proc_macro_derive(CssStyleMacro, attributes(short_prop, vendor_prefixes))]
pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let mut short_prop: Option<String> = None;
    let mut vendor_prefixes: Option<String> = None;

    // Iterate over the struct's #[...] attributes
    for option in input.attrs.into_iter() {
        let option = option.parse_meta().expect("option to exist");
        match option {
            // Match '#[ident = lit]' attributes. Match guard makes it '#[short_prop = lit]'
            Meta::NameValue(MetaNameValue {
                ref path, ref lit, ..
            }) if path.is_ident("short_prop") => {
                if let Lit::Str(lit) = lit {
                    short_prop = Some(lit.value());
                }
            }
            Meta::NameValue(MetaNameValue {
                ref path, ref lit, ..
            }) if path.is_ident("vendor_prefixes") => {
                if let Lit::Str(lit) = lit {
                    vendor_prefixes = Some(lit.value());
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
                fn #snake_case_type_ident<T>(self, val:T) -> Style  where T: UpdateStyle<#css_type_name> ;

                 #[track_caller]
                 fn #short_prop_ident<T>(self, val:T) -> Style  where T: UpdateStyle<#css_type_name>;
            ),
            quote!(

            #[track_caller]
            fn #snake_case_type_ident<T>(mut self, val:T) -> Style  where T: UpdateStyle<#css_type_name> {

                self.updated_at.push(format!("{}", Location::caller()));
                val.update_style(&mut self);

                self

             }

             #[track_caller]
             fn #short_prop_ident<T>(mut self, val:T) -> Style  where T: UpdateStyle<#css_type_name> {

                self.updated_at.push(format!("{}", Location::caller()));
                val.update_style(&mut self);
                 self
              }
            ),
        )
    } else {
        (
            quote!(

            #[track_caller]
            fn #snake_case_type_ident<T>(self, val:T) -> Style  where T: UpdateStyle<#css_type_name>;

            ),
            quote!(

            #[track_caller]
            fn #snake_case_type_ident<T>(mut self, val:T) -> Style  where T: UpdateStyle<#css_type_name> {

                self.updated_at.push(format!("{}", Location::caller()));
                val.update_style(&mut self);
                self
             }

            ),
        )
    };

    let mut outer_vendor_prefixes_quote = quote! {};

    if let Some(vendor_prefixes) = &vendor_prefixes {
        let prefixes = vendor_prefixes.split(",").map(|p| p.to_string());

        let inner_vendor_prefixes_quote = quote! {
            Some(vec![#(#prefixes.to_string()),*])

        };

        outer_vendor_prefixes_quote = quote! {
              fn prefixes(&self) -> Option<Vec<String>>{
                  #inner_vendor_prefixes_quote
            }

        };
    }

    let update_style_quote = quote! {
        impl CssValueTrait for #css_type_name{#outer_vendor_prefixes_quote}


    impl From<&str> for #css_type_name where {
        fn from(v: &str) -> Self {
            #css_type_name::StringValue(v.to_string())
        }
    }

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
                        fn #f_small_name_ident(self) -> Style;
                        fn #short_prop_ident(self) -> Style;

                    }
                } else {
                    quote! {
                        fn #f_small_name_ident(self) -> Style;
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
                        fn #short_prop_ident(mut self) -> Style {


                            self.updated_at.push(format!("{}", Location::caller()));
                            self.add_rule(Box::new(#css_type_name :: #f_big_name));
                            // #vendor_prefixes_quote
                            self
                        }
                        #[track_caller]
                            fn #f_small_name_ident(mut self) -> Style {


                            self.updated_at.push(format!("{}", Location::caller()));
                            self.add_rule(Box::new(#css_type_name :: #f_big_name));

                            self
                        }
                    }
                } else {
                    quote! {
                        #[track_caller]
                        fn #f_small_name_ident(mut self) -> Style {

                            self.updated_at.push(format!("{}", Location::caller()));
                            self.add_rule(Box::new(#css_type_name :: #f_big_name));

                            self
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

#[proc_macro_derive(CssPseudoMacro)]
pub fn expand_pseudo(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    if let syn::Data::Enum(data_enum) = input.data {
        let func_impls_defn = data_enum
            .variants
            .iter()
            .filter(|v| v.fields == syn::Fields::Unit)
            .map(|v| {
                let snake_case_variant =
                    format_ident!("{}", v.ident.clone().to_string().to_snake_case());
                quote! {
                    fn #snake_case_variant(self) -> Style;
                }
            });

        let trait_definition = quote! {
            pub trait PseudoTrait {
                #(
                #func_impls_defn
                )*

                fn before(self, before: &str)  -> Style;
                fn after(self, after: &str)  -> Style;
                fn lang( self, val: &str)  -> Style ;

                fn not( self, val: &str)  -> Style ;

                fn nth_child( self, val: usize)  -> Style ;


                fn nth_last_child( self, val: usize)  -> Style ;

                fn nth_last_of_type( self, val: usize)  -> Style ;


                fn nth_of_type( self, val: usize)  -> Style ;
            }
        };

        let func_impls = data_enum
            .variants
            .iter()
            .filter(|v| v.fields == syn::Fields::Unit)
            .map(|v| {
                let snake_case_variant =
                    format_ident!("{}", v.ident.clone().to_string().to_snake_case());
                let big_name = v.ident.clone();
                quote! {
                        #[track_caller]
                        fn #snake_case_variant(mut self) -> Style {

                            self.updated_at.push(format!("{}", Location::caller()));
                            self.pseudo = Pseudo::#big_name ;

                            self
                        }


                }
            });

        let trait_impl = quote! {

            impl PseudoTrait for Style {
                #(#func_impls)*

                fn before(mut self, before: &str)  -> Style {
                    self.updated_at.push(format!("{}", Location::caller()));
                    self.pseudo = Pseudo::Before(before.to_string());

                    self
                }

                fn after(mut self, after: &str)  -> Style {
                    self.updated_at.push(format!("{}", Location::caller()));
                    self.pseudo = Pseudo::After(after.to_string());

                    self
                }
                fn lang(mut self, val: &str)  -> Style {
                    self.updated_at.push(format!("{}", Location::caller()));
                    self.pseudo = Pseudo::Lang(val.to_string());

                    self
                }
                fn not(mut self, val: &str)  -> Style {
                    self.updated_at.push(format!("{}", Location::caller()));
                    self.pseudo = Pseudo::Not(val.to_string());

                    self
                }
                fn nth_child(mut self, val: usize)  -> Style {
                    self.updated_at.push(format!("{}", Location::caller()));
                    self.pseudo = Pseudo::NthChild(val);

                    self
                }

                fn nth_last_child(mut self, val: usize)  -> Style {
                    self.updated_at.push(format!("{}", Location::caller()));
                    self.pseudo = Pseudo::NthLastChild(val);

                    self
                }

                fn nth_last_of_type(mut self, val: usize)  -> Style {
                    self.updated_at.push(format!("{}", Location::caller()));
                    self.pseudo = Pseudo::NthLastOfType(val);

                    self
                }

                fn nth_of_type(mut self, val: usize)  -> Style {
                    self.updated_at.push(format!("{}", Location::caller()));
                    self.pseudo = Pseudo::NthOfType(val);

                    self
                }
            }
        };

        let exp = quote! {
            #trait_definition
            #trait_impl
        };

        exp.into()
    } else {
        let exp = quote! {};
        exp.into()
    }
}

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
// // Proc macro to generate psuedo methods for Style
// struct CreatePseudos {
//     properties: ExprArray,
// }

// impl Parse for CreatePseudos {
//     fn parse(input: ParseStream) -> Result<Self> {
//         let properties: ExprArray = input.parse()?;

//         Ok(CreatePseudos { properties })
//     }
// }

// #[proc_macro]
// pub fn create_pseudos(input: TokenStream) -> TokenStream {
//     let CreatePseudos { properties } = parse_macro_input!(input as CreatePseudos);

//     let mut exp = quote! {};
//     for property in properties.elems.iter() {
//         if let Expr::Lit(ref property) = property {
//             if let syn::Lit::Str(ref property) = property.lit {
//                 let pseudoname = format_ident!("{}", property.value());

//                 let fname = format_ident!("{}", property.value().to_snake_case());

//                 let expanded = quote! {

//                 #[track_caller]
//                 pub fn #fname(mut self) -> Style {

//                     self.updated_at.push(format!("{}", Location::caller()));
//                     self.pseudo = Pseudo::#pseudoname;
//                     self
//                 }

//                 };
//                 // println!("{}",expanded);
//                 exp = quote! {
//                     #exp
//                     #expanded
//                 };
//             }
//         }

//         // let elem_ident = Ident::from(elem);
//     }

//     // let exp = quote!{};
//     TokenStream::from(exp)
// }

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
                    let struct_type = format_ident!(
                        "ReturnSpecific{}From{}",
                        specific_type_names.value(),
                        theme_ids.value()
                    );
                    let expanded = quote! {



                        struct #struct_type<T: #themeid_ident + 'static>(T);

                        impl<T> ActOnIteratorOfThemes<Option<#specific_ident>> for #struct_type<T>
                        where
                            T: #themeid_ident + 'static,
                        {
                            fn call<'a, It>(&self, mut it:  It) -> Option<#specific_ident>
                            where
                                It: Iterator<Item = &'a Theme>,
                            {
                                it.find_map(|theme| theme.get::<T, #generic_ident>(self.0.clone())).map(|v| v.into())
                            }
                        }


                        impl ActOnIteratorOfThemes<Option<#specific_ident>> for ReturnThemeValFromUsize<#specific_ident>

                        {
                            fn call<'a, It>(&self, mut it:  It) -> Option<#specific_ident>
                            where
                                It: Iterator<Item = &'a Theme>,
                            {
                                it.find_map( |theme| theme.#theme_scale_ident.get(self.0.clone())).cloned().map(|v| v.into())
                            }
                        }

                            impl <T> From<T> for #specific_ident where T:#themeid_ident + 'static{
                                fn from(v: T) -> Self {
                                    with_themes( #struct_type(v)).unwrap()
                                }
                            }

                            impl <T>From<(T,#generic_ident)> for #specific_ident where T: #themeid_ident + 'static {
                                fn from(v:(T,#generic_ident)) -> Self {


                                    if let Some(theme_value) =  with_themes( #struct_type(v.0)){
                                        theme_value.clone()
                                    } else {
                                        v.1.clone().into()
                                    }

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

                                    let theme_value : #specific_ident =
                                        if let Some(theme_value) =   with_themes( ReturnThemeValFromUsize(self, PhantomData::<#specific_ident>)){
                                            theme_value.clone()

                                        } else {
                                            panic!("Theme scale does not exist {}", #theme_scale_string )

                                        };
                                    style.add_rule(Box::new(theme_value));
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




                                    if let Some(bp_scale) = with_themes(ReturnBpScale) {


                                        let mut old_style = None;

                                        for (style_idx, bp) in bp_scale.iter().enumerate(){
                                            if let Some(theme_idx) = self.get(style_idx){
                                            if let Some(generic_value )= with_themes(ReturnThemeValFromUsize(*theme_idx, PhantomData::<#specific_ident>)) {

                                                    let specific_value : #specific_ident = generic_value.into();

                                                    let rules = style
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
                                                    let rules = style
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
// use darling::FromMeta;
// use syn::AttributeArgs;

// #[derive(Debug, FromMeta)]
// struct MacroArgs {
//     msg_type: syn::Ident,
// }

fn get_arg_name(fnarg : &FnArg) -> String {
match fnarg {
        FnArg::Receiver(_) => panic!("cannot be a method with self receiver"),
        FnArg::Typed(t) => {
            match &*t.pat {
                Pat::Ident(syn::PatIdent { ident, .. }) => ident.to_string(), //syn::parse_quote!(&#ident),
                _ => unimplemented!("Only supported on simplest argument expressions"),
            }
        }
}
}


fn get_single_type_name(fnarg : &FnArg) -> String {
    match fnarg {
            FnArg::Receiver(_) => panic!("cannot be a method with self receiver"),
            FnArg::Typed(t) => {
                // panic!("{:#?}", t);
                match &*t.ty {
                    syn::Type::Path(syn::TypePath { path, .. }) => {
                        // panic!("{:#?}", path);
                        let path = path.get_ident().expect("Are you sure you have passed in an argument struct");
                        path.to_string()
                    }, //syn::parse_quote!(&#ident),
                    _ => unimplemented!("Only supported on simplest argument expressions"),
                }
            }
    }
    }


fn get_node_msg_type_name(fnarg : &FnArg) -> String {
    match fnarg {
            FnArg::Receiver(_) => panic!("cannot be a method with self receiver"),
            FnArg::Typed(t) => {
                // panic!("{:#?}", t);
                match &*t.ty {
                    syn::Type::Path(syn::TypePath { path, .. }) => {
                        let segment = path.segments.last().expect("segment to exist");
                        match &segment.arguments {
                            syn::PathArguments::AngleBracketed(generic_args) => {
                                match generic_args.args.first().expect("generic args to exist"){
                                    syn::GenericArgument::Type(syn::Type::Path(type_path)) => {
                                        let ident = type_path.path.get_ident().expect("generic argument ident to exist");
                                        ident.to_string()
                                    }
                                    _ => unimplemented!()
                                }
                                //     syn::Type::Path(path) => {}
                                //     _ => unimplemented!(),
                                // }
                            }
                            _ => unimplemented!()
                        }

                        
                        // let path = path.get_ident().unwrap();
                        
                        // path.to_string();
                    
                    
                    }, //syn::parse_quote!(&#ident),
                    _ => unimplemented!("Only supported on simplest argument expressions"),
                }
            }
    }
    }

 
fn is_option_type(fnarg : &FnArg) -> bool {
    match fnarg {
            FnArg::Receiver(_) => panic!("cannot be a method with self receiver"),
            FnArg::Typed(t) => {
                // panic!("{:#?}", t);
                match &*t.ty {
                    syn::Type::Path(syn::TypePath { path, .. }) => {
                        let segment = path.segments.last().expect("segment to exist");
                        // panic!("{:#?}", segment);
                        if segment.ident.to_string() != "Node" && segment.ident.to_string() != "Option"  && segment.ident.to_string() != "Vec"     {
                            panic!("Node type is not Node, Vec<Node> or Option")
                        }

                        if segment.ident.to_string() == "Option" {
                            match &segment.arguments {
                                syn::PathArguments::AngleBracketed(generic_args) => {
                                    match generic_args.args.first().expect("generic args to exist"){
                                        syn::GenericArgument::Type(syn::Type::Path(type_path)) => {
                                            if type_path.path.segments.first().expect("option to have a type").ident.to_string()!="Node" {
                                                panic!("Option does not contain a Node")
                                            }
                                        }
                                        _ => unimplemented!()
                                    }
                                    //     syn::Type::Path(path) => {}
                                    //     _ => unimplemented!(),
                                    // }
                                }
                                _ => unimplemented!()
                            }
                            true
                        } else {
                            if segment.ident.to_string() != "Node" &&  segment.ident.to_string() != "Vec"{
                                panic!("argument type is not Node or Vec")
                            }
                            false
                        }

            
                        // let path = path.get_ident().unwrap();
                        
                        // path.to_string();
                    
                    
                    }, //syn::parse_quote!(&#ident),
                    _ => unimplemented!("Only supported on simplest argument expressions"),
                }
            }
    }
    }
    
     
fn is_vec_type(fnarg : &FnArg) -> bool {
    match fnarg {
            FnArg::Receiver(_) => panic!("cannot be a method with self receiver"),
            FnArg::Typed(t) => {
                match &*t.ty {
                    syn::Type::Path(syn::TypePath { path, .. }) => {
                        let segment = path.segments.last().expect("segment to exist");
                        // panic!("{:#?}", segment);
                        if segment.ident.to_string() != "Node" && segment.ident.to_string() != "Vec" {
                            panic!("Node type is not Node or Vec")
                        }

                        if segment.ident.to_string() == "Vec" {
                            match &segment.arguments {
                                syn::PathArguments::AngleBracketed(generic_args) => {
                                    match generic_args.args.first().expect("generic args to exist"){
                                        syn::GenericArgument::Type(syn::Type::Path(type_path)) => {
                                            if type_path.path.segments.first().expect("Vec to have a type").ident.to_string()!="Node" {
                                                panic!("Vec does not contain a Node")
                                            }
                                        }
                                        _ => unimplemented!()
                                    }
                                    //     syn::Type::Path(path) => {}
                                    //     _ => unimplemented!(),
                                    // }
                                }
                                _ => unimplemented!()
                            }
                            true
                        } else {
                            if segment.ident.to_string() != "Node" && segment.ident.to_string() != "Option"{
                                panic!("argument type is not Node or Option")
                            }
                            false
                        }
                    
                    }, 
                    _ => unimplemented!("Only supported on simplest argument expressions"),
                }
            }
    }
    }
    
    
    fn assert_children_have_vec_structure(fnarg : &FnArg) {
        match fnarg {
                FnArg::Receiver(_) => panic!("cannot be a method with self receiver"),
                FnArg::Typed(t) => {
                    // panic!("{:#?}", t);
                    match &*t.ty {
                        syn::Type::Path(syn::TypePath { path, .. }) => {
                            let segment = path.segments.last().expect("segment to exist");
                            // panic!("{:#?}", segment);
                            if segment.ident.to_string() != "Vec" {
                                panic!("Children need to have Vec<Node<_>> type!")
                            } else {
                                match &segment.arguments {
                                    syn::PathArguments::AngleBracketed(generic_args) => {
                                        match generic_args.args.first().expect("generic args to exist"){
                                            syn::GenericArgument::Type(syn::Type::Path(type_path)) => {
                                                if type_path.path.segments.first().expect("vec to have a type").ident.to_string()!="Node" {
                                                    panic!("Vec does not contain a Node")
                                                }
                                            }
                                            _ => unimplemented!()
                                        }
                                    }
                                    _ => unimplemented!()
                                }
                            }
                        
                        }, 
                        _ => unimplemented!("Only supported on simplest argument expressions"),
                    }
                }
        }
        }   
#[proc_macro_attribute]
pub fn view_macro(_args: TokenStream, input: TokenStream) -> TokenStream {
    
    let mut input_fn: ItemFn = syn::parse_macro_input!(input);
    
    if !input_fn.sig.ident.to_string().ends_with("_view") {
        panic!("Your function name needs to end with _view");
    }
    let input_fn_ident = input_fn.sig.ident.clone();

    let input_fn_string = input_fn.sig.ident.to_string();
    let view_ident_string = input_fn_string.trim_end_matches("_view");

    let view_ident = format_ident!("{}", view_ident_string);

    // Names of the root and children view function arguments
    // Initially not underscore prefixed as assumed to be utilized.

    let mut root_ident = format_ident!("root");
    // let mut children_ident = format_ident!("children");
    let mut children_used = true;

    // Allow us to iter over the view arguments in turn.
    let has_args = if let Some(first_arg) = &input_fn.sig.inputs.first(){
        if get_arg_name(first_arg) == "root" || get_arg_name(first_arg) == "_root"  {
            false
        } else {
            true
        }
    } else {
        panic!("There does not appear to be a first argument")
    };

    let mut input_iter = &mut input_fn.sig.inputs.iter();

    let mut view_args_ident = None;
    if has_args {
        if let Some(view_args_fn_arg) = input_iter.next(){
            view_args_ident = Some(format_ident!("{}",get_single_type_name(view_args_fn_arg)))
        } else {
            panic!("Cannot determine the type for view arguments. If your view does not have any arguments pass the () type.")
        };
    }

    let msg_type_ident = if let Some(root_fn_arg) = input_iter.next(){
        
        let argument_name = get_arg_name(root_fn_arg);
        if (argument_name != "root") && (argument_name != "_root") {
            panic!("the first Node<Ms> argument must be root or _root")
        }
        if argument_name == "_root" {
            root_ident = format_ident!("_root");
        }
        format_ident!("{}",get_node_msg_type_name(root_fn_arg))

    } else {
        panic!("Cannot determine the Msg type for this seed view, are you sure you are returning Node<MsgType>>")
    };

    let view_builder = format_ident!("{}Builder", view_ident_string.to_camel_case());

    if let Some(children_arg) = input_iter.next(){
        let argument_name = get_arg_name(children_arg);
        if (argument_name != "children") && (argument_name != "_children") {
            panic!("The argument after root must be 'children' or '_children'")
        }

        assert_children_have_vec_structure(children_arg);

        if argument_name == "_children" {
            children_used = false;
        }

    };

    let mut vec_of_args = vec![];
    let mut vec_of_vec_args = vec![];
    let mut vec_of_all_args = vec![];
    let mut vec_of_optional_args = vec![];

    for input in &mut input_iter {
        let arg_name = format_ident!("{}",get_arg_name(input));

        vec_of_all_args.push( arg_name.clone() );
        
        vec_of_args.push( (arg_name.clone() , is_option_type(input), is_vec_type(input)  ) );

        if is_option_type(input) {
            vec_of_optional_args.push( arg_name.clone() );
        }

        if is_vec_type(input) {
            vec_of_vec_args.push( arg_name.clone() );
        }
    }

    

    let mut view_builder_inner_quote = quote! { root: Node<#msg_type_ident>,};
    let mut view_builder_empty_impl_inner_quote = quote! {root: div![],};
    let mut view_function_call_impl_inner_quote = if has_args {
        quote! {self.args, self.root, children, }
    } else {
        quote! {self.root, children, }
    };

    for (name, optional, is_a_vec) in vec_of_args.iter() {
    
        let new_line = match (*optional, *is_a_vec) {
            (true, false) => quote! {  #name: Option<Node<#msg_type_ident>>,},
            (false, false) => quote! {  #name: Node<#msg_type_ident>,},
            (false, true) => quote! {  #name: Vec<Node<#msg_type_ident>>,},
            (true, true) => panic!("You should never have an optional vec arg"),
        };

        view_builder_inner_quote = quote! {
            #view_builder_inner_quote
            #new_line
        };

        let new_line = match (*optional, *is_a_vec) {
            (true, false) =>   quote! {  #name: None,},
            (false, false) =>  quote! {  #name: empty![],},
            (false, true) =>  quote! {  #name: vec![],},
            (true, true) => panic!("You should never have an optional vec arg"),
        };


        view_builder_empty_impl_inner_quote = quote! {
            #view_builder_empty_impl_inner_quote
            #new_line
        };

        // let new_line = if *optional {
        //     quote! {  self.#name,}
        // } else {
        //     quote! {  self.#name, }
        // };

        let new_line = quote!{ self.#name, };

        view_function_call_impl_inner_quote = quote! {
            #view_function_call_impl_inner_quote
            #new_line
        };
    }

    let view_render_impl_quote = if children_used {
        quote!(
            let children = 
            match &mut self.root {
                seed::virtual_dom::node::Node::Element(ref mut el) => {
                    std::mem::replace(&mut el.children, vec![])
                }
                seed::virtual_dom::node::Node::Text(ref mut text) => {
                    let cow  = std::borrow::Cow::<'static, str>::default();
                    let node_cow = std::mem::replace(&mut text.text, cow);

                    vec![span![node_cow.to_string()]]
                }
                seed::virtual_dom::node::Node::Empty => {
                    vec![]
                }
            };
        )
    } else {
     quote!(
        let children = vec![];
     )   
    };

    let mut args_quote = quote!();
    let mut args_impl_quote = quote!();

    if has_args {
        let view_args_ident = view_args_ident.clone().unwrap();
        args_quote = quote!(args: #view_args_ident,);
        args_impl_quote = quote!(args: #view_args_ident::default(),);
    }
   
    let view_builder_quote = quote! {
        struct #view_builder<#msg_type_ident> where #msg_type_ident : 'static {
            #args_quote
            #view_builder_inner_quote
        }

        impl <#msg_type_ident> #view_builder<#msg_type_ident>   where #msg_type_ident : 'static {
            fn default_empty() -> #view_builder::<#msg_type_ident>{
                #view_builder::<#msg_type_ident> {
                    #args_impl_quote
                    #view_builder_empty_impl_inner_quote
                }
            }

            fn render(mut self) -> Node<#msg_type_ident> { 
                
                #view_render_impl_quote
        
                #input_fn_ident(
                    #view_function_call_impl_inner_quote
                )
            }

            fn update_el(self, elc: &mut El<#msg_type_ident>) {
                self.render().update_el(elc);
            }
        }
    };

  
    let view_trait_impls_quote = quote! {};
   
    let mut args_macro_quote = quote! {};

    for (name, _optional , _is_a_vec) in vec_of_args.iter() {
        let name = format_ident!("{}_{}", view_ident, name);
        
        let macro_item_impl_quote = quote! {
            #[allow(unused_macros)]
        
           macro_rules! #name {
                ( $($ part:tt)* ) => {
                   {
                       #[allow(unused_mut)]
                       let mut eld = El::empty(seed::virtual_dom::Tag::Div);
                       process_submacro_part!([$($part)*]); 
                       Node::Element(eld)
                   }
               };
           }
        };

        args_macro_quote = quote! {
           #args_macro_quote
           #macro_item_impl_quote
        };
    }


    let main_view_macro_name_view_ident = format_ident!("MainViewMacroName_{}", view_ident);
    let macro_impl_quote = quote! {


    #args_macro_quote

    #[macro_export]
    #[allow(unused_macros)]
    macro_rules! #view_ident {
        ( $($ part:tt)* ) => {
                {
                    #[allow(unused_mut)]
                    let mut builder = #view_builder::<_>::default_empty();
                        
                    process_part!([#main_view_macro_name_view_ident, [#(#vec_of_all_args),*], [#(#vec_of_optional_args),*] , [#(#vec_of_vec_args),*] , [$($part)*]]) ;
                            
                    builder.render()
                }
            };
    }
    };

    let inner_block = input_fn.block;
    input_fn.block = syn::parse_quote! {{

    #[allow(unused_macros)]
    macro_rules! root {
        ( $($ part:expr),* $(,)? ) => {

                {
                    let mut root = #root_ident;
                    $ (
                        let  val = $ part;
                        match root {
                            seed::virtual_dom::Node::Element(ref mut elx) => {
                                val.update_el(elx)
                        },
                        _ => {}
                    }
                    )*

            root
        }
        };
    }



        #inner_block
    }
    };

    quote::quote!(
        #view_builder_quote
        #view_trait_impls_quote
        #macro_impl_quote
        #[allow(dead_code)]
        #input_fn
    )
    .into()
}


struct ProcessPartArray{
    parts: Vec<ProcessPart>,
}


enum ProcessPart {
    Macro(ProcessPartMacro),
    Assign(ProcessPartAssign),
    Neither(ProcessPartNeither),
}

struct ProcessPartAssign {
    left_ident: syn::Ident,
    right: Box<Expr>,
}

struct ProcessPartMacro {
    main_name_ident: syn::Ident,
    ident: syn::Ident,
    optional: bool,
    is_a_vec: bool,
    tokens: TokenStream,
}

struct ProcessPartNeither {
    expr: Expr,
}

impl Parse for ProcessPartArray {
    fn parse(input: ParseStream) -> Result<Self> {
        // an array is passed to the process_part macro.
        let args :ExprArray = input.parse()?;

        // we need to iterate over it,  the first element is the main macro name
        // the next element is a list of all arguments
        // the next element is a list of optional arguments
        // the final element is the actual expression being processed.
        let mut iter_for_args = args.elems.iter();
        
        let macro_name_expr = iter_for_args.next().cloned().expect("macro_name_expr to exist");
        
        // all_args and optional_args are needed when processing a macro expression.
        let all_args = iter_for_args.next().cloned().expect("This ExprArray should exist");
        let optional_args = iter_for_args.next().cloned().expect("This ExprArray should exist");
        let vec_args = iter_for_args.next().cloned().expect("This ExprArray should exist");

        let expr = iter_for_args.next().cloned().expect("expression to exist");
        

        let main_macro_name_ident = match macro_name_expr {
            Expr::Path(path) => {
                        
                let ident =  path.path.get_ident().expect("MainViewMacroName to exist");
                    let ident_string = ident.to_string();
                    if ident_string.starts_with("MainViewMacroName_"){
                        format_ident!("{}",ident_string.trim_start_matches("MainViewMacroName_").to_string())
                } else {
                    panic!("Cannot determine main macro name")
                }
            }
            _ => {panic!("cannot determine main macro name")}
        };

        // The main expression can be an assign, a macro, or something else
        // if its an assign macro we construct a struct that enables optional arguments to be set
        // if its a macro we construct a struct that enables the correct macro to be called
        // if its something else we allow update the root element.

        if let  Expr::Array(expr_array) = expr {
            let mut vec_of_processed_parts  = vec![];

        
        for expr in expr_array.elems {
        match expr {  
            
                    Expr::Assign(expr_assign) => {
                        let left_ident = if let Expr::Path(path) = *expr_assign.left {
                            let path = path.path;
                            path.clone().get_ident().clone().expect("left ident to exist").clone()
                        } else {
                            unimplemented!()
                        };

                        vec_of_processed_parts.push(ProcessPart::Assign(ProcessPartAssign {
                            left_ident: left_ident.clone(),
                            right: expr_assign.right,
                        }))
                    }
                    Expr::Macro(expr_macro) => {
                        let path = expr_macro.mac.path.clone();

                        let ident = path.clone().get_ident().clone().expect("expr_macro inside Expr::Macro to exist").clone();
                        let tokens = expr_macro.mac.tokens.clone();

                        // if ident is not included in all_args then it is not an argument

                        let is_an_argument_macro  = if match &all_args {
                            Expr::Array(expr_array) => {
                                expr_array.elems.iter().any(|item| 
                                    if let Expr::Path(path) = item {
                                        path.path.get_ident().unwrap().to_string() == ident.to_string() 
                                    } else {
                                        unimplemented!()
                                    }
                                )
                            }
                            _ => unimplemented!()
                        } {
                            true
                        } else {
                            false
                        };


                        let optional = if match &optional_args {
                            Expr::Array(expr_array) => {
                                expr_array.elems.iter().any(|item| 

                                    if let Expr::Path(path) = item {
                                        path.path.get_ident().unwrap().to_string() == ident.to_string() 
                                    } else {
                                        unimplemented!()
                                    }
                                )
                            }
                            _ => unimplemented!()
                        } {
                            true
                        } else {
                            false
                        };

                        let is_a_vec = if match &vec_args {
                            Expr::Array(expr_array) => {
                                expr_array.elems.iter().any(|item| 

                                    if let Expr::Path(path) = item {
                                        path.path.get_ident().unwrap().to_string() == ident.to_string() 
                                    } else {
                                        unimplemented!()
                                    }
                                )
                            }
                            _ => unimplemented!()
                        } {
                            true
                        } else {
                            false
                        };


                    if is_an_argument_macro {
                        vec_of_processed_parts.push(ProcessPart::Macro(ProcessPartMacro {
                            main_name_ident: main_macro_name_ident.clone(),
                            ident: ident,
                            optional,
                            is_a_vec,
                            tokens: tokens.into(),
                        }))
                        // let global_macro_ident = format_ident!("{}_{}",global_view_name , macro_ident);
                    }
                     else {
                        vec_of_processed_parts.push(ProcessPart::Neither(ProcessPartNeither {
                            expr: Expr::Macro(expr_macro)
                        }))
                     }
                    }
                    exp => {
                        // not a macro or an assign, therefore we construct a struct 
                        // to enable the root element to be updated.

                        vec_of_processed_parts.push(ProcessPart::Neither(ProcessPartNeither {
                            expr: exp.clone(),
                        }))
                    }
                }
            }
            Ok(ProcessPartArray{
                parts: vec_of_processed_parts,
            })
           
        }
        else {
            panic!("dsdsd")
        }
     
    
}
}

#[proc_macro]
pub fn process_part(input: TokenStream) -> TokenStream {
    
    let processed_part_array = parse_macro_input!(input as ProcessPartArray);
    let mut combined_quote = quote!();

    for part in processed_part_array.parts {    
        match part {
    
        ProcessPart::Macro(m) => {
            let macro_name = m.ident.clone();
        let global_macro_ident = format_ident!("{}_{}", m.main_name_ident, m.ident);
        let tokens: proc_macro2::TokenStream = m.tokens.into();
        
        let exp = match (m.optional, m.is_a_vec){
            (true, false) =>  quote!(   builder.#macro_name = Some(#global_macro_ident![#tokens]); ),
            (false, false) =>   quote!(     builder.#macro_name = #global_macro_ident![#tokens]; ),
            (false, true) => quote!(   builder.#macro_name.push(#global_macro_ident![#tokens]); ),
            (true, true) => panic!("You should never have an optional vec arg"),
        };
           

        combined_quote = quote!(
            #combined_quote
            #exp
        );
        }
        ProcessPart::Assign(assign) => {
            let left_ident = assign.left_ident;
        let expr = assign.right;
            combined_quote = quote!(
                #combined_quote
                builder.args.#left_ident = #expr;
            )
        }
        ProcessPart::Neither(ProcessPartNeither{expr}) => {   
            combined_quote = quote!(
                #combined_quote
                let can_update_root = #expr;
                match &mut builder.root {
                seed::virtual_dom::Node::Element(ref mut ela) => {
                    can_update_root.update_el( ela);
                },
                    _ => {}
                }
            )
        }
    }
    
    }
    // panic!("{:#?}", combined_quote);
    quote!({
        #combined_quote
    }).into()
    // quote!().into()
}


struct AsElem {
    elem_name_ident : syn::Ident,
    affected_node_ident : syn::Ident,
    exprs : Vec<Expr>,
}


impl Parse for AsElem {
    fn parse(input: ParseStream) -> Result<Self> {
        let elem_name_ident = input.parse::<syn::Ident>()?;
        let _comma = input.parse::<syn::token::Comma>()?;
        let affected_node_ident = input.parse::<syn::Ident>()?;
        let mut exprs = vec![];
        if let Ok(_comma) = input.parse::<syn::token::Comma>(){
            

            let mut done = false;
            while !done {
                if let Ok(expr) = input.parse::<syn::Expr>(){
                    exprs.push(expr);
                } else {
                    done = true;
                }
                if let Ok(_comma) = input.parse::<syn::token::Comma>(){

                } else {
                    done = true
                }
            }
        }

        Ok(
            AsElem {
                elem_name_ident,
                affected_node_ident,
                exprs,
            }
        )
    }
}



#[proc_macro]
pub fn as_tag(input: TokenStream) -> TokenStream {
    let as_tag = parse_macro_input!(input as AsElem);

    let affected_node_ident = as_tag.affected_node_ident;
    let elem_name_ident = format_ident!("{}",as_tag.elem_name_ident.to_string().to_camel_case());
    let exprs = as_tag.exprs.iter();

    let mut exprs_quote = quote!();

    for expr in exprs {
        exprs_quote = quote!({
            #exprs_quote
            #expr.update_el(ela);
        });
    }
    

    let exp = quote!({
            match #affected_node_ident {
                seed::virtual_dom::Node::Element(ref mut ela) => {
                    ela.tag = Tag::#elem_name_ident;
                    #exprs_quote
                }
                _ => panic!("cannot use as_tag with Text or empty nodes")
            }

            #affected_node_ident
    });

   exp.into()
    
}


struct ProcessSubMacroPartArray{
    parts: Vec<ProcessSubMacroPart>,
}



enum ProcessSubMacroPart {
    Assign(ProcessSubMacroPartAssign),
    Neither(ProcessSubMacroPartNeither),
}

struct ProcessSubMacroPartAssign {
    left_ident: syn::Ident,
    right: Box<Expr>,
}

struct ProcessSubMacroPartNeither {
    expr: Expr,
}

impl Parse for ProcessSubMacroPartArray {
    fn parse(input: ParseStream) -> Result<Self> {
        let expr_array :ExprArray = input.parse()?;
        let mut vec_of_processed_parts  = vec![];
        for expr in expr_array.elems {
            match expr {  

            
                    Expr::Assign(expr_assign) => {
                        let left_ident = if let Expr::Path(path) = *expr_assign.left {
                            let path = path.path;
                            path.clone().get_ident().clone().expect("left ident to exist").clone()
                        } else {
                            unimplemented!()
                        };

                        vec_of_processed_parts.push(ProcessSubMacroPart::Assign(ProcessSubMacroPartAssign {
                            left_ident: left_ident.clone(),
                            right: expr_assign.right,
                        }))
                    }
                    exp => {
                        // not a macro or an assign, therefore we construct a struct 
                        // to enable the root element to be updated.

                        vec_of_processed_parts.push(ProcessSubMacroPart::Neither(ProcessSubMacroPartNeither {
                            expr: exp.clone(),
                        }))
                    }
                }
            }
            
            Ok(ProcessSubMacroPartArray{
                parts: vec_of_processed_parts,
            })
    
}
}

#[proc_macro]
pub fn process_submacro_part(input: TokenStream) -> TokenStream {
    let processed_part_array = parse_macro_input!(input as ProcessSubMacroPartArray);
    let mut combined_quote = quote!();

      for part in processed_part_array.parts {    
        match part {
            ProcessSubMacroPart::Assign(assign) => {
                let left_ident = assign.left_ident;
                let expr = assign.right;
                let exp = quote!(
                    builder.args.#left_ident = #expr;
                );
                combined_quote = quote!(
                    #combined_quote
                    #exp
                )
            }
            ProcessSubMacroPart::Neither(ProcessSubMacroPartNeither{expr}) => {   
                let exp = quote!(
                    #expr.update_el( &mut eld);
                );

                combined_quote = quote!(
                    #combined_quote
                    #exp
                )
            }
    }
}
    quote!({
        #combined_quote
    }).into()
}
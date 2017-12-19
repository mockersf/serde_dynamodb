extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use std::collections::HashMap;

use proc_macro::TokenStream;
use syn::*;
use syn::NestedMetaItem::MetaItem;
use syn::MetaItem::NameValue;
use syn::MetaItem::List;

#[proc_macro_derive(Expression, attributes(serde_dynamodb))]
pub fn expression(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_macro_input(&s).unwrap();
    let name = &ast.ident;
    let ty_params = &ast.generics.ty_params;
    let gen: quote::Tokens = match ast.body {
        Body::Enum(_) => panic!("#[derive(Expression)] is only defined for structs, not enums"),
        Body::Struct(fields) => impl_get_expression(name, fields, ty_params)
    };
    gen.parse().unwrap()
}

fn impl_get_expression(name: &Ident, variant_data: VariantData, ty_params: &[TyParam]) -> quote::Tokens {
    let conditions: HashMap<Ident, String> = variant_data.fields().iter().map(|field| {
        let field_name = format!("{}", field.ident.clone().unwrap());

        let mut rhs = field_name.clone();
        for meta_items in field.attrs.iter().filter_map(|meta_items| get_meta_items_from("serde", meta_items)) {
            for meta_item in meta_items {
                match meta_item {
                    // Parse `#[serde(rename = "foo")]`
                    MetaItem(NameValue(ref name, ref lit)) if name == "rename" => {
                        let s = get_string_from_lit(name.as_ref(), name.as_ref(), lit);
                        rhs = s.clone();
                    },
                    _ => {}
                }
            }
        }
        let mut lhs = field_name.clone();
        for meta_items in field.attrs.iter().filter_map(|meta_items| get_meta_items_from("serde_dynamodb", meta_items)) {
            for meta_item in meta_items {
                match meta_item {
                    // Parse `#[serde_dynamodb(name = "foo")]`
                    MetaItem(NameValue(ref name, ref lit)) if name == "name" => {
                        let s = get_string_from_lit(name.as_ref(), name.as_ref(), lit);
                        lhs = s.clone();
                    },
                    _ => {}
                }
            }
        }

        return (field.ident.clone().unwrap(), format!("{} = {}", lhs, rhs));
    }).collect();
    let types = get_type_params(ty_params);
    let type_params = quote! { <#(#types)*> };
    let fields = conditions.iter().map(|(ident, expr)| {
        quote! {
            match self.#ident {
                Some(_) => Some(#expr),
                None => None
            }
        }
    });
    quote! {
        impl#type_params Expression for #name#type_params {
            fn get_expression(self: &#name#type_params) -> String {
                let present_conditions = vec![
                    #(#fields),*
                ];
                present_conditions.iter().filter_map(|v| v.map(|v| v.to_string())).collect::<Vec<String>>().join(" and ")
            }
            fn to_query_input(&self, table: String) -> QueryInput {
                QueryInput {
                    table_name: table,
                    expression_attribute_values: Some(serde_dynamodb::to_hashmap(self).unwrap()),
                    key_condition_expression: Some(self.get_expression()),
                    ..Default::default()
                }
            }
        }
    }
}

fn get_type_params(ty_params: &[TyParam]) -> Vec<quote::Tokens> {
    ty_params.iter().map(|ty_param| {
        let type_ident = &ty_param.ident;
        quote! {
            #type_ident,
        }
    }).collect()
}

fn get_string_from_lit(
    attr_name: &str,
    meta_item_name: &str,
    lit: &syn::Lit,
) -> String {
    if let syn::Lit::Str(ref s, _) = *lit {
        s.clone()
    } else {
        panic!(
            format!(
                "expected {} attribute to be a string: `{} = \"...\"`",
                attr_name,
                meta_item_name
            )
        );
    }
}

fn get_meta_items_from(domain: &str, attr: &syn::Attribute) -> Option<Vec<syn::NestedMetaItem>> {
    match attr.value {
        List(ref name, ref items) if name == domain => Some(items.iter().cloned().collect()),
        _ => None,
    }
}

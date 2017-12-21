extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::*;
use syn::NestedMetaItem::MetaItem;
use syn::MetaItem::NameValue;
use syn::MetaItem::List;


fn impl_build_query_input(
    name: &Ident,
    visibility: &Visibility,
    variant_data: VariantData,
    ty_params: &[TyParam],
) -> quote::Tokens {

    let types = get_type_params(ty_params);
    let type_params = quote! { <#(#types)*> };
    let fields_as_options: Vec<Field> = variant_data
        .fields()
        .iter()
        .map(|field| {

            // wrap type in an option to allow filtering only on some fields
            let original_type: &syn::Ty = &field.ty;
            let optional_type = Ty::Path(
                None,
                syn::Path {
                    global: false,
                    segments: vec![
                        syn::PathSegment {
                            ident: Ident::new("Option"),
                            parameters: PathParameters::AngleBracketed(
                                AngleBracketedParameterData {
                                    lifetimes: vec![],
                                    bindings: vec![],
                                    types: vec![original_type.clone()],
                                }
                            ),
                        },
                    ],
                },
            );

            // rename fields to ":field_name" to match syntax in dynamodb expressions
            let mut renamed_attributes: Vec<syn::Attribute> = field
                .attrs
                .iter()
                .map(|meta_items| {
                    let new_value = match meta_items.clone().value {
                        List(ref name, ref items) if name == "serde" => {
                            let mut new_items: Vec<syn::NestedMetaItem> = items
                                .iter()
                                .cloned()
                                .map(|meta_item| match meta_item {
                                    MetaItem(NameValue(ref name, ref lit)) if name == "rename" => {
                                        let old_rename = get_string_from_lit(name.as_ref(), lit);
                                        let new_rename = format!(":{}", old_rename);
                                        MetaItem(NameValue("rename".into(), new_rename.into()))
                                    }
                                    other => other,
                                })
                                .collect();
                            if new_items
                                .iter()
                                .cloned()
                                .filter(|meta_item| match *meta_item {
                                    MetaItem(NameValue(ref name, _)) if name == "rename" => true,
                                    _ => false,
                                })
                                .count() == 0
                            {
                                new_items.push(MetaItem(NameValue(
                                    "rename".into(),
                                    format!(":{}", field.ident.clone().unwrap()).into(),
                                )));
                            }
                            List(name.clone(), new_items)
                        }
                        other => other,
                    };
                    Attribute {
                        value: new_value,
                        ..meta_items.clone()
                    }
                })
                .collect();
            if renamed_attributes
                .iter()
                .cloned()
                .filter(|meta_items| match meta_items.value {
                    List(ref name, _) if name == "serde" => true,
                    _ => false,
                })
                .count() == 0
            {
                renamed_attributes.push(Attribute {
                    value: List(
                        "serde".into(),
                        vec![
                            MetaItem(NameValue(
                                "rename".into(),
                                format!(":{}", field.ident.clone().unwrap()).into(),
                            )),
                        ],
                    ),
                    style: AttrStyle::Outer,
                    is_sugared_doc: false,
                })
            }

            let mut optional_field = field.clone();
            optional_field.ty = optional_type;
            optional_field.attrs = renamed_attributes;
            optional_field
        })
        .collect();

    let fields_as_conditions: Vec<quote::Tokens> = variant_data
        .fields()
        .iter()
        .map(|field| {
            let field_name = format!("{}", field.ident.clone().unwrap());

            let mut renamed = field_name.clone();
            for meta_items in field.attrs.iter().filter_map(|meta_items| {
                get_meta_items_from("serde", meta_items)
            })
            {
                for meta_item in meta_items {
                    match meta_item {
                        // Parse `#[serde(rename = "foo")]`
                        MetaItem(NameValue(ref name, ref lit)) if name == "rename" => {
                            let s = get_string_from_lit(name.as_ref(), lit);
                            renamed = s.clone();
                        }
                        _ => {}
                    }
                }
            }

            let ident = &field.ident;
            let condition = format!("{} = :{}", renamed, renamed);
            quote! {
                match self.#ident {
                    Some(_) => Some(#condition),
                    None => None,
                }
            }
        })
        .collect();

    let query_input_name = Ident::from(format!("{}QueryInput", name));
    quote! {
        #[derive(Default, Serialize)]
        #visibility struct #query_input_name#type_params {
            #(#fields_as_options),*
        }

        impl#type_params #query_input_name#type_params {
            fn get_expression(&self) -> String {
                let present_conditions = vec![
                    #(#fields_as_conditions),*
                ];
                present_conditions
                    .iter()
                    .filter_map(|v| v.map(|v| v.to_string()))
                    .collect::<Vec<String>>()
                    .join(" and ")
            }
        }

        impl #type_params ToQueryInput for #query_input_name#type_params {
            fn to_query_input(&self, table_name: String) -> QueryInput {
                QueryInput {
                    table_name: table_name,
                    expression_attribute_values: Some(serde_dynamodb::to_hashmap(self).unwrap()),
                    key_condition_expression: Some(self.get_expression()),
                    ..Default::default()
                }
            }
        }
    }
}

fn get_type_params(ty_params: &[TyParam]) -> Vec<quote::Tokens> {
    ty_params
        .iter()
        .map(|ty_param| {
            let type_ident = &ty_param.ident;
            quote! {
            #type_ident,
        }
        })
        .collect()
}

fn get_string_from_lit(attr_name: &str, lit: &syn::Lit) -> String {
    if let syn::Lit::Str(ref s, _) = *lit {
        s.clone()
    } else {
        panic!(format!(
            "expected {} attribute to be a string",
            attr_name,
        ));
    }
}

fn get_meta_items_from(domain: &str, attr: &syn::Attribute) -> Option<Vec<syn::NestedMetaItem>> {
    match attr.value {
        List(ref name, ref items) if name == domain => Some(items.iter().cloned().collect()),
        _ => None,
    }
}

#[proc_macro_derive(ToQueryInput)]
pub fn query_input_macro(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_macro_input(&s).unwrap();
    let name = &ast.ident;
    let visibility = &ast.vis;
    let ty_params = &ast.generics.ty_params;
    let gen: quote::Tokens = match ast.body {
        Body::Enum(_) => panic!("#[derive(ToQueryInput)] is only defined for structs, not enums"),
        Body::Struct(fields) => impl_build_query_input(name, visibility, fields, ty_params),
    };
    gen.parse().unwrap()
}

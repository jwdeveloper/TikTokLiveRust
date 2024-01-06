use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use proc_macro2::{Ident, TokenStream};

use quote::{quote, ToTokens};
use syn::{Field, Fields, File, Item};

pub struct TikTokEventDataModel
{
    pub is_webcast: bool,
    pub webcast_name: String,
    pub enum_name: String,
    pub event_name: String,
    pub raw_name: String,
    pub method_name: String,

    pub fields: Vec<TokenStream>,

    pub webcast_name_ident: Ident,
    pub enum_name_ident: Ident,
    pub event_name_ident: Ident,
    pub method_name_ident: Ident,
}


pub fn get_event_data_models() -> Vec<TikTokEventDataModel>
{
    let mut webcast_models = get_models_from_proto();
    let custom_models = get_models_from_custom_event();


    for custom_model in custom_models
    {
        let custom_model_name =  &custom_model.0.to_string();
        let custom_model_fields= custom_model.1.fields.clone();
        if !webcast_models.contains_key(custom_model_name)
        {
            webcast_models.insert(custom_model.0, custom_model.1);
            continue;
        }

        match webcast_models.get_mut(custom_model_name)
        {
            Some(mut value) =>
                {
                    for field in custom_model_fields
                    {
                        value.fields.push(field.clone());
                    }
                }
            None => println!("'key3' is not in the map"),
        }
    }


    return webcast_models.into_values().collect();
}


fn remove_duplicates(models: &mut Vec<TikTokEventDataModel>) {
    let mut unique_event_names = HashSet::new();
    models.retain(|model| unique_event_names.insert(model.event_name.clone()));
}


fn get_models_from_proto() -> HashMap<String, TikTokEventDataModel>
{
    let syntax_tree = get_syntax_tree("src/proto/webcast.rs".to_string());
    let mut eventModels = HashMap::new();
    for item in syntax_tree.items
    {
        if let Item::Struct(struct_item) = item
        {
            let webcast_name = struct_item.ident.to_string();
            let raw_name = webcast_name.clone()
                .replace("Webcast", "")
                .replace("Message", "");

            let enum_name = format!("On{}", raw_name);
            let event_name = format!("TikTok{}Event", raw_name).to_string();
            let method_name = camel_to_snake_case(enum_name.clone().as_str());

            let event_name_ident = Ident::new(&*event_name, proc_macro2::Span::call_site());
            let webcast_name_ident = Ident::new(&*webcast_name, proc_macro2::Span::call_site());
            let enum_name_ident = Ident::new(&*enum_name, proc_macro2::Span::call_site());
            let method_name_ident = Ident::new(&*method_name, proc_macro2::Span::call_site());


            let methodName = quote!
            {
                 pub raw_data: #webcast_name_ident,
            };

            let model = TikTokEventDataModel
            {
                fields: vec![methodName],
                is_webcast: true,
                webcast_name,
                enum_name,
                event_name: event_name.clone(),
                raw_name,
                method_name,

                webcast_name_ident,
                enum_name_ident,
                event_name_ident,
                method_name_ident,
            };
            eventModels.insert(event_name.clone().to_string(), model);
        }
    }
    return eventModels;
}

fn get_models_from_custom_event() -> HashMap<String, TikTokEventDataModel>
{
    let syntax_tree = get_syntax_tree("src/event_models.rs".to_string());
    let mut eventModels = HashMap::new();
    for item in syntax_tree.items
    {
        if let Item::Struct(struct_item) = item
        {
            let name = struct_item.ident.to_string();
            let raw_name = name.clone()
                .replace("TikTok", "")
                .replace("Event", "");

            let enum_name = format!("On{}", raw_name);
            let event_name = format!("TikTok{}Event", raw_name).to_string();
            let method_name = camel_to_snake_case(enum_name.clone().as_str());

            let event_name_ident = Ident::new(&*event_name, proc_macro2::Span::call_site());
            let webcast_name_ident = Ident::new(&*name, proc_macro2::Span::call_site());
            let enum_name_ident = Ident::new(&*enum_name, proc_macro2::Span::call_site());
            let method_name_ident = Ident::new(&*method_name, proc_macro2::Span::call_site());

            let fields_tokenstream = match struct_item.fields {
                Fields::Named(fields) => format_fields(fields.named.into_iter()),
                Fields::Unnamed(fields) => format_fields(fields.unnamed.into_iter()),
                Fields::Unit => quote! {}, // For unit structs, which have no fields
            };
            let model = TikTokEventDataModel
            {
                fields: vec![fields_tokenstream],
                is_webcast: false,
                webcast_name: name,
                enum_name,
                event_name: event_name.clone(),
                raw_name,
                method_name,

                webcast_name_ident,
                enum_name_ident,
                event_name_ident,
                method_name_ident,
            };
            eventModels.insert(event_name.clone().to_string(), model);
        }
    }
    return eventModels;
}

//""
fn get_syntax_tree(path: String) -> File
{
    let path = Path::new(path.as_str());
    let code = fs::read_to_string(path).expect("Unable to read the file");
    let syntax_tree = syn::parse_file(&code).expect("Unable to parse file");
    return syntax_tree;
}

fn camel_to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, char) in s.chars().enumerate() {
        if char.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(char.to_lowercase().next().unwrap());
    }
    result
}

fn format_fields(fields: impl Iterator<Item = Field>) -> proc_macro2::TokenStream {
    let formatted_fields: Vec<proc_macro2::TokenStream> = fields
        .map(|f| {
            let field_tokens = f.into_token_stream();
            quote! { #field_tokens, }
        })
        .collect();

    quote! {  #( #formatted_fields )* }
}
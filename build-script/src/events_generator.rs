use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use crate::CODE_EVENTS_OUTPUT_PATH;
use crate::event_models_loader::TikTokEventDataModel;

pub fn generate_events_class(models: &Vec<TikTokEventDataModel>)
{
    let mut structures = HashMap::new();
    let mut enumProps = vec![];
    for model in models
    {
        let event_name_indent = &model.event_name_ident;
        let enum_name_indent = &model.enum_name_ident;
        let webcast_name_ident = &model.webcast_name_ident;

        if structures.contains_key(&model.event_name)
        {
            continue;
        }

        let enumProp: TokenStream = quote! {
            #enum_name_indent(#event_name_indent),
        };

        let fields = model.fields.clone()
            .into_iter()
            .fold(TokenStream::new(), |mut acc, ts| {
                acc.extend(ts);
                acc
            });

        let structure = quote! {
          pub struct #event_name_indent
          {
              #fields
          }
        };

        println!("STRUCT {}",structure);
        structures.insert(model.event_name.clone(), structure);
        enumProps.push(enumProp);
    }


    let combinedStructs = structures
        .into_iter()
        .fold(TokenStream::new(), |mut acc, ts| {
            acc.extend(ts.1.clone());
            acc
        });

    let combinedEnums = enumProps
        .into_iter()
        .fold(TokenStream::new(), |mut acc, ts| {
            acc.extend(ts);
            acc
        });

    let parse_message = quote! {

        use crate::generated::messages::webcast::*;
        use crate::generated::messages::webcast::webcast_response::Message;
        ///
        /// Generated file
        ///

        #combinedStructs

        pub enum TikTokLiveEvent
        {
            #combinedEnums
        }
        };

    let binding = parse_message.to_string();
    let code = binding.as_str();

    let syntax_tree = syn::parse_file(code).unwrap();
    let formatted = prettyplease::unparse(&syntax_tree);


    let mut file = File::create(CODE_EVENTS_OUTPUT_PATH.to_owned() + "events.rs").unwrap();
    file.write_all(formatted.as_bytes()).unwrap();
}

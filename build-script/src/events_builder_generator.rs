use crate::event_models_loader::TikTokEventDataModel;
use crate::CODE_EVENTS_OUTPUT_PATH;
use proc_macro2::TokenStream;
use quote::quote;
use std::fs::File;
use std::io::Write;

pub fn generate_builder_class(models: &Vec<TikTokEventDataModel>) {
    let mut methods = vec![];
    for model in models {
        let event_name_indent = &model.event_name_ident;
        let enum_name_indent = &model.enum_name_ident;
        let method_name_indent = &model.method_name_ident;

        let event_method: TokenStream = quote! {
               pub fn #method_name_indent(&mut self, on_event: fn(client: &TikTokLiveClient, event_data: &#event_name_indent)) -> &mut Self
        {
            self.on_event(|client, incoming_event|
                {
                    match incoming_event
                    {
                        TikTokLiveEvent::#enum_name_indent(event_instance) =>
                            {
                                on_event(client,event_instance);
                            }
                               _ => {}
                    }
                })
        }
            };
        methods.push(event_method);
    }

    let combined_methods = methods.into_iter().fold(TokenStream::new(), |mut acc, ts| {
        acc.extend(ts);
        acc
    });

    let file_content: TokenStream = quote! {
     use crate::core::live_client::TikTokLiveClient;
     use crate::core::live_client_builder::TikTokLiveBuilder;
     use crate::generated::events::*;


     ///
     ///  Generated code
     ///
     ///
     impl TikTokLiveBuilder
     {
         #combined_methods
     }

    };

    let binding = file_content.to_string();
    let code = binding.as_str();

    let syntax_tree = syn::parse_file(code).unwrap();
    let formatted = prettyplease::unparse(&syntax_tree);

    let mut file = File::create(CODE_EVENTS_OUTPUT_PATH.to_owned() + "events_builder.rs").unwrap();
    file.write_all(formatted.as_bytes()).unwrap();
}

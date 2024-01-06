use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use crate::CODE_EVENTS_OUTPUT_PATH;
use crate::event_models_loader::TikTokEventDataModel;

pub fn generate_mapper_class(models: &Vec<TikTokEventDataModel>)
{
    let mut matchCases = vec![];
    for model in models
    {
        if model.is_webcast == false
        {
            continue;
        }

        let event_name_indent = &model.event_name_ident;
        let enum_name_indent = &model.enum_name_ident;
        let webcast_name_ident = &model.webcast_name_ident;
        let webcast_name = &model.webcast_name;


        let match_case: TokenStream = quote! {
            #webcast_name =>
                {
                    let raw_data = #webcast_name_ident::parse_from_bytes(proto_message_content).unwrap();
                    let event = #event_name_indent
                    {
                        raw_data
                    };
                    client.publish_event(TikTokLiveEvent::#enum_name_indent(event));
                }

        };

        matchCases.push(match_case);
    }


    let combinedCases = matchCases
        .into_iter()
        .fold(TokenStream::new(), |mut acc, ts| {
            acc.extend(ts);
            acc
        });

    let parse_message = quote! {

        use protobuf::Message;
        use crate::core::live_client::TikTokLiveClient;
        use crate::generated::messages::webcast::*;
        use crate::core::live_client_mapper::TikTokLiveMessageMapper;
        use crate::generated::events::*;

impl TikTokLiveMessageMapper
{

    pub fn handle_single_message(&self, message: crate::generated::messages::webcast::webcast_response::Message, client: &TikTokLiveClient)
    {
        let proto_message_name = &message.method;
        let proto_message_content = &message.payload;
        match proto_message_name.as_str()
        {
            #combinedCases
            _ =>
                {
                    client.publish_event(TikTokLiveEvent::OnWebsocketUnknownMessage(TikTokWebsocketUnknownMessageEvent
                    {
                        message_name: message.method.clone(),
                        raw_data: message,
                    }));
                }
        }
    }
}
        };

    let binding = parse_message.to_string();
    let code = binding.as_str();

    let syntax_tree = syn::parse_file(code).unwrap();
    let formatted = prettyplease::unparse(&syntax_tree);


    let mut file = File::create(CODE_EVENTS_OUTPUT_PATH.to_owned() + "events_mapper.rs").unwrap();
    file.write_all(formatted.as_bytes()).unwrap();
}

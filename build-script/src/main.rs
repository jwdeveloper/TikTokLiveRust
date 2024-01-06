mod event_models;
mod events_builder_generator;
mod event_models_loader;
mod events_generator;
mod events_mapper_generator;

pub mod proto;

use std::collections::HashMap;
use std::fmt::{Debug, format};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::Duration;
use proc_macro2::{Ident, TokenStream};
use protobuf_codegen::Codegen;
use quote::quote;
use reqwest::Client;
use syn::Item;
use crate::events_builder_generator::generate_builder_class;
use crate::event_models_loader::{get_event_data_models};
use crate::events_generator::generate_events_class;
use crate::events_mapper_generator::generate_mapper_class;

const PROTO_OUTPUT_PATH: &str = "proto/";
const CODE_OUTPUT_PATH: &str = "../src/generated/messages";

const CODE_EVENTS_OUTPUT_PATH: &str = "../src/generated/";

#[tokio::main]
async fn main()
{
    let path = Path::new(PROTO_OUTPUT_PATH);
    fs::create_dir_all(path).unwrap();

    let names = ["data", "enums", "webcast"];
    handle_download_proto_files(names).await;
    handle_generating_proto(names);

    let event_models = get_event_data_models();

    generate_events_class(&event_models);
    generate_builder_class(&event_models);
    generate_mapper_class(&event_models);
}



async fn handle_download_proto_files(names: [&str; 3])
{
    let base_url = "https://raw.githubusercontent.com/jwdeveloper/TikTokLiveJava/master/API/src/main/proto/";
    let client = Client::builder()
        .timeout(Duration::from_secs(3))
        .build()
        .unwrap();

    let urls: HashMap<String, String> = names.into_iter()
        .map(|u| (u.to_string(), base_url.to_owned() + u.clone() + ".proto"))
        .collect();

    for url in urls
    {
        let proto_url = url.1;
        let proto_name = PROTO_OUTPUT_PATH.to_string() + &*url.0 + ".proto";

        let content = handle_file_download(&proto_url, &client).await;
        let mut file = File::create(proto_name).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }
}


fn handle_generating_proto(names: [&str; 3])
{
    let mut builder = &mut Codegen::new();
    builder = builder.protoc()
        .protoc_path(&protoc_bin_vendored::protoc_bin_path().unwrap())
        .includes(&[PROTO_OUTPUT_PATH]);
    for name in names
    {
        let file = PROTO_OUTPUT_PATH.to_string() + &name + ".proto";
        builder = builder.input(file)
    }
    builder.out_dir(CODE_OUTPUT_PATH).run_from_script();
}

async fn handle_file_download(url: &String, client: &Client) -> String
{
    let response = client.get(url).send().await.unwrap();
    if !response.status().is_success()
    {
        false;
    }
    return response.text().await.unwrap();
}

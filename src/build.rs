use protobuf_codegen::Codegen;

fn main()
{
    println!("hello from build");
    generate_proto()
}


fn generate_proto()
{

    let names = ["data","enums","webcast"];
    let base_url = "https://raw.githubusercontent.com/jwdeveloper/TikTokLiveJava/master/API/src/main/proto/";
    let urls : Vec<String> = names.iter()
        .map(|a| base_url.to_owned() + a.to_owned() +".proto" )
        .collect();

    urls.iter().for_each(|x| println!("{}", x.to_owned()));



   Codegen::new()
        .protoc()
        .protoc_path(&protoc_bin_vendored::protoc_bin_path().unwrap())
        .includes(&["src/proto"])
        .input("src/proto/webcast.proto")
        .input("src/proto/data.proto")
        .input("src/proto/enums.proto")
        .out_dir("src/proto/messages")
        .run_from_script();
}
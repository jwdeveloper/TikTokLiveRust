use tiktoklive_client;
use tiktoklive_client::tiktok_live::TikTokLive;

fn main()
{
    let client = TikTokLive::new_client()
        .configure(|settings|
            {

                println!("siema!")
            })
        .configure(|settings|
            {

                println!("dupa!")
            })
        .on_message(|event|
            {})
        .build();


    client.connect();
}
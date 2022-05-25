use std::sync::Arc;

use sanity_rs_client::{config::SanityConfig, sanity_client::SanityClient};

// Have to manually create a blocking tokio context for upload_image function to work
async fn upload_image(client: Arc<SanityClient>) {
    let client = Arc::clone(&client);
    let response = tokio::task::spawn_blocking(move || {
        let res = client.upload_image(String::from("image.png"));
        println!("{:?}", res);
        return res;
    }).await.unwrap();
}

#[tokio::main]
async fn main() {
    let id = "";
    let dataset = "";
    let api_version = "";

    let config = SanityConfig::new(id, dataset)
        .api_version(api_version)
        .access_token("")
        .build();

    let s_client = SanityClient::new(config);

    let arc_client = Arc::new(s_client);

    let upload = upload_image(arc_client).await;
}

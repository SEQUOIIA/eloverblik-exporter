mod config;
mod error;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let conf = config::load_conf().unwrap();
    let client = eloverblik_client::new_builder()
        .add_config(eloverblik_client::Config {
            refresh_token: conf.eloverblik_refresh_token
        })
        .build();

    let token_resp = client.auth().await.unwrap();
    println!("{:?}", token_resp);
}

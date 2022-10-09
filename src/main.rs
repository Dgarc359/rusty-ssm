use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ssm::{Client, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");

    let config = aws_config::from_env().region(region_provider).load().await;

    let client = Client::new(&config);

    let resp = client
        .get_parameters()
        .set_names(Some(
            ["CHAMELEON_BOT_APP_ID", "CHAMELEON_BOT_TOKEN"]
                .map(String::from)
                .to_vec()
        ))
        .with_decryption(true)
        .send()
        .await?;

    println!("resp: {:?}", &resp.parameters().into_iter().enumerate());

    Ok(())
}

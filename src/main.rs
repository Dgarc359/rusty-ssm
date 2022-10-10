use aws_config::{meta::region::RegionProviderChain, SdkConfig};
use aws_sdk_ssm::{Client, Error};

async fn build_ssm_client() -> SsmWrapperClient {
    let region_provider: RegionProviderChain = RegionProviderChain::default_provider().or_else("us-east-1");

    let config: SdkConfig = aws_config::from_env().region(region_provider).load().await;

    SsmWrapperClient {
        client: Client::new(&config) 
    }
}

struct SsmWrapperClient {
    client: Client,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = build_ssm_client().await.client;

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

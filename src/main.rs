use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ssm::{Client, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");

    let config = aws_config::from_env().region(region_provider).load().await;

    let client = Client::new(&config);
    
    // Im pretty sure there's a better way to do multiple parameters at once...
    // Docs: https://docs.rs/aws-sdk-ssm/latest/aws_sdk_ssm/client/fluent_builders/struct.GetParameters.html
    let resp = client.get_parameters().names(String::from("CHAMELEON_BOT_APP_ID")).names(String::from("CHAMELEON_BOT_TOKEN")).with_decryption(true).send().await?;
    println!("resp: {:?}", &resp.parameters().into_iter().enumerate());

    Ok(())
}

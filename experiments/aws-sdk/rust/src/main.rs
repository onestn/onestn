use aws_sdk_s3::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let client = Client::new(&config);

    let resp = client
        .list_objects_v2()
        .bucket("kbds-s3-ailabs")
        .send()
        .await?;

    for obj in resp.contents() {
        println!(
            "{} ({} bytes)",
            obj.key().unwrap_or_default(),
            obj.size().unwrap_or(0)
        );
    }
    Ok(())
}

use anyhow::{Context, Result, bail};
use std::path::{Path, PathBuf};

#[derive(serde::Deserialize)]
struct Config {
    mappings: Vec<Mapping>,
}

#[derive(serde::Deserialize)]
struct Mapping {
    local_prefix: String,
    bucket: String,
    #[serde(default)]
    key_prefix: Option<String>,
    #[serde(default = "default_true")]
    strip_prefix: bool,
}
fn default_true() -> bool {
    true
}

fn find_config(start: &Path) -> Result<(PathBuf, Config)> {
    for dir in start.ancestors().skip(1) {
        let candidate = dir.join(".s3deploy.toml");
        if candidate.exists() {
            let text = std::fs::read_to_string(&candidate)?;
            let config: Config = toml::from_str(&text).context("failed to parse .s3deploy.toml")?;
            return Ok((dir.to_path_buf(), config));
        }
    }
    bail!("no .s3deploy.toml found in any parent directory")
}

fn resolve(config: &Config, rel: &Path) -> Result<(String, String)> {
    for m in &config.mappings {
        if let Ok(stripped) = rel.strip_prefix(&m.local_prefix) {
            let inner = if m.strip_prefix { stripped } else { rel };
            let key = match &m.key_prefix {
                Some(p) => format!("{}/{}", p, inner.display()),
                None => inner.display().to_string(),
            };
            return Ok((m.bucket.clone(), key));
        }
    }
    bail!("no mapping matches path: {}", rel.display())
}

#[tokio::main]
async fn main() -> Result<()> {
    let file: PathBuf = std::env::args()
        .nth(1)
        .context("usage: s3deploy <file>")?
        .into();
    let file = file.canonicalize()?;

    let (root, config) = find_config(&file)?;
    let rel = file.strip_prefix(&root)?;
    let (bucket, key) = resolve(&config, rel)?;

    let aws = aws_config::load_defaults(aws_config::BehavitorVersion::latest()).await;
    let client = aws_sdk_s3::Client::new(&aws);

    let body = aws_sdk_s3::primitives::ByteStream::from_path(&file).await?;
    client
        .put_oject()
        .bucket(&bucket)
        .key(&key)
        .body(body)
        .send()
        .await
        .with_context(|| format!("upload failed: s3://{bucket}/{key}"))?;

    println!("deployed -> s3://{bucket}/{key}");
    Ok(())
}

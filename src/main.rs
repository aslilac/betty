use once_cell::sync::Lazy;
use std::fs;
use std::path::Path;

static CLIENT: Lazy<reqwest::Client> = Lazy::new(|| reqwest::Client::new());

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let files = toml::from_str::<toml::Value>(&fs::read_to_string("files.toml")?)?;

	for (local, remote) in files
		.as_table()
		// .ok_or_else(|| Err(anyhow!("files.toml format is invalid")))?
		.unwrap()
	{
		let local_path = Path::new(local);
		// Assume existing files are up to date
		if local_path.exists() {
			continue;
		}

		// Create missing parent directories
		if let Some(local_dir) = local_path.parent() {
			if !local_dir.exists() {
				fs::create_dir_all(local_path.parent().unwrap())?;
			}
		}

		// Get file
		println!("fetching {}", local);
		let content = CLIENT
			.get(remote.as_str().unwrap())
			.send()
			.await?
			.bytes()
			.await?;
		fs::write(local_path, content)?;
	}

	Ok(())
}

use serde::Deserialize;
use std::{collections::HashMap, fs, path::PathBuf};

#[derive(Deserialize)]
struct Config {
    username: String,
    password: String,
    domain: String,
    #[serde(default = "default_output")]
    output: PathBuf,
}

fn default_output() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_default()
        .join(".local/share/gazeta.opml")
}

#[derive(Deserialize)]
struct Folder {
    id: u64,
    name: String,
}

#[derive(Deserialize)]
struct Feed {
    url: String,
    title: String,
    #[serde(rename = "folderId")]
    folder_id: Option<u64>,
}

#[derive(Deserialize)]
struct FoldersResponse {
    folders: Vec<Folder>,
}

#[derive(Deserialize)]
struct FeedsResponse {
    feeds: Vec<Feed>,
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = dirs::config_dir()
        .unwrap_or_default()
        .join("gazeta/config.toml");

    let config: Config = toml::from_str(
        &fs::read_to_string(&config_path)
            .map_err(|_| format!("config not found at {config_path:?}"))?
    )?;

    let base = format!(
        "{}/index.php/apps/news/api/v1-3",
        config.domain.trim_end_matches('/')
    );

    let client = reqwest::blocking::Client::new();

    let folders: FoldersResponse = client
        .get(format!("{base}/folders"))
        .basic_auth(&config.username, Some(&config.password))
        .send()?.json()?;

    let feeds: FeedsResponse = client
        .get(format!("{base}/feeds"))
        .basic_auth(&config.username, Some(&config.password))
        .send()?.json()?;

    let mut by_folder: HashMap<u64, Vec<&Feed>> = HashMap::new();
    let mut root_feeds: Vec<&Feed> = Vec::new();

    for feed in &feeds.feeds {
        match feed.folder_id {
            Some(fid) => by_folder.entry(fid).or_default().push(feed),
            None => root_feeds.push(feed),
        }
    }

    let mut opml = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
         <opml version=\"1.0\">\n  \
           <head><title>Nextcloud News</title></head>\n  \
           <body>\n"
    );

    for folder in &folders.folders {
        let name = escape_xml(&folder.name);
        opml.push_str(&format!("    <outline text=\"{name}\" title=\"{name}\">\n"));
        for feed in by_folder.get(&folder.id).unwrap_or(&vec![]) {
            let title = escape_xml(&feed.title);
            let url = escape_xml(&feed.url);
            opml.push_str(&format!(
                "      <outline type=\"rss\" text=\"{title}\" title=\"{title}\" xmlUrl=\"{url}\"/>\n"
            ));
        }
        opml.push_str("    </outline>\n");
    }

    for feed in &root_feeds {
        let title = escape_xml(&feed.title);
        let url = escape_xml(&feed.url);
        opml.push_str(&format!(
            "    <outline type=\"rss\" text=\"{title}\" title=\"{title}\" xmlUrl=\"{url}\"/>\n"
        ));
    }

    opml.push_str("  </body>\n</opml>\n");

    if let Some(parent) = config.output.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&config.output, &opml)?;
    println!("wrote {} feeds to {:?}", feeds.feeds.len(), config.output);

    Ok(())
}

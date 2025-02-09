use clap::Parser;
use indexmap::IndexMap;
use indicatif::{ProgressBar, ProgressStyle};
use inquire::Select;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(next_line_help = true)]
struct VscerCli {
    /// extension id, eg: `charliermarsh.ruff`
    #[arg(long, short = 'i')]
    ext_id: String,
    /// number of latest versions to display, default is 10
    #[arg(long, short = 'n', default_value_t = 10)]
    n_latest: u16,
    /// directory to save the vsix file, default is current directory (`.`)
    #[arg(long, short = 'o', default_value = ".", value_hint = clap::ValueHint::DirPath)]
    outdir: PathBuf,
}

type VersionMap = IndexMap<String, HashMap<String, String>>;

fn group_by_version(ext_versions: Vec<ExtVersion>) -> VersionMap {
    let mut grouped: VersionMap = IndexMap::new();

    for item in ext_versions {
        if let Some(target_platform) = item.target_platform {
            grouped
                .entry(item.version.clone())
                .or_insert_with(HashMap::new)
                .insert(target_platform, item.file_source);
        }
    }
    grouped
}

fn fetch_extension_query(ext_id: &String) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder().build()?;

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Accept",
        "application/json;api-version=3.0-preview.1".parse()?,
    );
    headers.insert("Content-Type", "application/json".parse()?);

    let data = json!({
        "filters": [
            {
                "criteria": [
                    {
                        "filterType": 7,
                        "value": ext_id
                    }
                ]
            }
        ],
        "flags": 103
    });
    let response = client
        .request(
            reqwest::Method::POST,
            "https://marketplace.visualstudio.com/_apis/public/gallery/extensionquery",
        )
        .headers(headers)
        .json(&data)
        .send()?;
    let body = response.text()?;
    Ok(body)
}

#[derive(Debug)]
struct ExtVersion {
    version: String,
    target_platform: Option<String>,
    file_source: String,
}

fn parse_extension_query_data(
    raw_text: String,
) -> Result<Vec<ExtVersion>, Box<dyn std::error::Error>> {
    let mut ext_versions: Vec<ExtVersion> = Vec::new();
    let raw_json: Value = serde_json::from_str(&raw_text)?;
    let versions = &raw_json["results"][0]["extensions"][0]["versions"];
    if versions.is_array() {
        let version_array = versions.as_array().unwrap();
        for v in version_array {
            let files = v["files"].as_array().unwrap();
            let vsix_source: Vec<_> = files
                .iter()
                .filter(|f| {
                    f["assetType"].as_str().unwrap()
                        == "Microsoft.VisualStudio.Services.VSIXPackage"
                })
                .collect();
            if vsix_source.len() < 1 {
                continue;
            }
            let ver = ExtVersion {
                version: v["version"].as_str().unwrap().to_string(),
                target_platform: v["targetPlatform"].as_str().map(|s| s.to_string()),
                file_source: vsix_source[0]["source"].as_str().unwrap().to_string(),
            };
            ext_versions.push(ver);
        }
    }

    Ok(ext_versions)
}

fn download_vsix_package(
    ext_id: String,
    vsix_url: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_name = format!("{}.vsix", ext_id);
    let client = reqwest::blocking::Client::builder().build()?;
    let mut file = std::fs::File::create(file_name)?;
    let response = client.request(reqwest::Method::GET, vsix_url).send()?;
    let content = response.bytes()?;
    file.write_all(&content)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = VscerCli::parse();

    let fetch_query_pb = ProgressBar::new_spinner();
    fetch_query_pb.set_style(ProgressStyle::with_template("{spinner:.green} {msg}").unwrap());
    fetch_query_pb.set_message("Fetching extension query...");
    fetch_query_pb.enable_steady_tick(Duration::from_millis(100));

    let extension_query = fetch_extension_query(&cli.ext_id)?;
    fetch_query_pb.finish_with_message("Fetched!");
    let ret = parse_extension_query_data(extension_query)?;
    if ret.is_empty() {
        println!("No extension found");
        exit(0)
    }
    let vsix_url = match ret[0].target_platform {
        Some(_) => {
            let grouped = group_by_version(ret);
            let versions: Vec<String> = grouped
                .keys()
                .take(cli.n_latest as usize)
                .cloned()
                .collect();
            let version_ans = Select::new("Select version", versions).prompt()?;
            let target_version = grouped.get(&version_ans).unwrap();
            let platform_options: Vec<String> = target_version.keys().cloned().collect();
            let platform_ans = Select::new("Select your platform", platform_options).prompt()?;
            let file_source = grouped
                .get(&version_ans)
                .unwrap()
                .get(&platform_ans)
                .unwrap();
            Some(file_source.to_string())
        }
        None => {
            let versions: Vec<String> = ret.iter().map(|v| v.version.clone()).collect();
            let version_ans = Select::new("Select version", versions).prompt()?;
            if let Some(ext) = ret.iter().find(|v| v.version == version_ans) {
                Some(ext.file_source.to_string())
            } else {
                None
            }
        }
    };
    if let Some(url) = vsix_url {
        let download_extension_pb = ProgressBar::new_spinner();
        download_extension_pb
            .set_style(ProgressStyle::with_template("{spinner:.green} {msg}").unwrap());
        download_extension_pb.set_message("Downloading ...");
        download_extension_pb.enable_steady_tick(Duration::from_millis(12));
        download_vsix_package(cli.ext_id, url)?;
        download_extension_pb.finish_with_message("Downloaded!");
    }

    Ok(())
}

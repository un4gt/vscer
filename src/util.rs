use crate::types::{ExtVersion, SelectMode, VersionMap};
use indexmap::IndexMap;
use indicatif::{ProgressBar, ProgressStyle};
use inquire::Select;
use serde_json::Value;
use std::borrow::Cow;
use std::collections::HashMap;
use std::time::Duration;

pub fn group_by_version(ext_versions: Vec<ExtVersion>) -> VersionMap {
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

pub fn parse_extension_query_data(
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

pub fn with_progress_bar<F, T, C>(
    start_message: C,
    done_message: C,
    f: F,
) -> Result<T, Box<dyn std::error::Error>>
where
    F: FnOnce() -> Result<T, Box<dyn std::error::Error>>,
    C: Into<Cow<'static, str>>,
{
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::with_template("{spinner:.green} {msg}")?);
    pb.set_message(start_message);
    pb.enable_steady_tick(Duration::from_millis(100));

    let result = f().unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });
    pb.finish_with_message(done_message);

    Ok(result)
}

pub fn handle_with_platform(
    grouped_query_data: &VersionMap,
    select_mode: SelectMode,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    match select_mode {
        SelectMode::SpecVersion(sv) => {
            if let Some(tv) = grouped_query_data.get(&sv) {
                let platform_options: Vec<String> = tv.keys().cloned().collect();
                let platform_ans =
                    Select::new("Select your platform", platform_options).prompt()?;
                let file_source = tv.get(&platform_ans).unwrap();
                return Ok(Some(file_source.to_string()));
            }
            Ok(None)
        }
        SelectMode::NLatest(latest) => {
            let versions: Vec<String> = grouped_query_data.keys().take(latest).cloned().collect();
            let version_ans = Select::new("Select version", versions).prompt()?;
            let target_version = grouped_query_data.get(&version_ans).unwrap();
            let platform_options: Vec<String> = target_version.keys().cloned().collect();
            let platform_ans = Select::new("Select your platform", platform_options).prompt()?;
            let file_source = grouped_query_data
                .get(&version_ans)
                .unwrap()
                .get(&platform_ans)
                .unwrap();
            Ok(Some(file_source.to_string()))
        }
    }
}

pub fn handle_without_platform(
    query_data: &Vec<ExtVersion>,
    select_mode: SelectMode,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    match select_mode {
        SelectMode::SpecVersion(sv) => {
            let target_version = query_data.iter().find(|v| v.version == sv);
            if let Some(tv) = target_version {
                return Ok(Some(tv.file_source.clone()));
            }
            Ok(None)
        }
        SelectMode::NLatest(latest) => {
            let versions: Vec<String> = query_data
                .iter()
                .take(latest)
                .map(|v| v.version.clone())
                .collect();
            let version_ans = Select::new("Select version", versions).prompt()?;
            if let Some(ext) = query_data.iter().find(|v| v.version == version_ans) {
                return Ok(Some(ext.file_source.clone()));
            }
            Ok(None)
        }
    }
}

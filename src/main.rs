use clap::Parser;

use cli::VscerCli;
use fetch::*;
use types::SelectMode;
use util::*;

mod cli;
mod fetch;
mod types;
mod util;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = VscerCli::parse();

    let extension_query = with_progress_bar("Fetching extension query...", "Fetched!", || {
        fetch_extension_query(&cli.ext_id)
    })?;
    let query_data = parse_extension_query_data(extension_query)?;
    if query_data.is_empty() {
        eprintln!("Extension query data is empty!");
        std::process::exit(1);
    }

    let with_target_platform = query_data[0].target_platform.is_some();

    let select_mode = match (cli.mode.n_latest, cli.mode.spec_version) {
        (Some(n), None) => SelectMode::NLatest(n as usize),
        (None, Some(s)) => SelectMode::SpecVersion(s),
        _ => SelectMode::NLatest(10usize),
    };

    let vsix_url = if with_target_platform {
        let grouped = group_by_version(query_data);
        handle_with_platform(&grouped, select_mode)
    } else {
        handle_without_platform(&query_data, select_mode)
    }?;

    match vsix_url {
        Some(url) => {
            with_progress_bar("Start to download...", "Downloaded!", || {
                download_vsix_package(&cli.ext_id, &url)
            })?;
        }
        None => {
            eprintln!("Extension's vsix url not found!");
            std::process::exit(1);
        }
    }

    Ok(())
}

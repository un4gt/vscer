use serde_json::json;
use std::io::Write;

pub fn fetch_extension_query(ext_id: &String) -> Result<String, Box<dyn std::error::Error>> {
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

pub fn download_vsix_package(
    ext_id: &String,
    vsix_url: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_name = format!("{}.vsix", ext_id);
    let client = reqwest::blocking::Client::builder().build()?;
    let mut file = std::fs::File::create(file_name)?;
    let response = client.request(reqwest::Method::GET, vsix_url).send()?;
    let content = response.bytes()?;
    file.write_all(&content)?;
    Ok(())
}

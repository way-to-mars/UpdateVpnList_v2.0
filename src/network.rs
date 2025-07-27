use std::fs;
use reqwest::get;
use reqwest::header::HeaderMap;

pub(crate) async fn fetch_raw_data(url: String) -> Result<String, String> {
    match get(url).await {
        Ok(response) => match response.text().await {
            Ok(text) => Ok(text),
            Err(e) => Err(e.to_string()),
        },
        Err(e) => Err(e.to_string()),
    }
}

// Returns (filename, is_updated)
pub(crate) async fn save_to_file(url: &String) -> Result<(String, bool), String> {
    match get(url).await {
        Ok(response) => {
            let file_name =
                get_name_from_headers(response.headers()).unwrap_or(generate_name_from_url(url));

            let file_existed = fs::exists(&file_name).unwrap_or(false);

            match response.bytes().await {
                Ok(bytes) => match fs::write(&file_name, bytes) {
                    Ok(_) => Ok((file_name, file_existed)),
                    Err(e) => Err(format!("Ошибка записи файла. {e}")),
                },
                Err(e) => Err(format!("Ошибка скачивания файла. {e}")),
            }
        }
        Err(e) => Err(format!("Ошибка скачивания файла. {e}")),
    }
}

fn get_name_from_headers(headers: &HeaderMap) -> Option<String> {
    // Searching for the header like that:
    // content-disposition: attachment; filename="profile.ovpn"
    match headers.get("content-disposition") {
        None => {}
        Some(header_value) => match header_value.to_str() {
            Err(_) => {}
            Ok(str_value) => {
                for sub_str in str_value.split(";") {
                    if sub_str.contains("filename=") {
                        let parts: Vec<&str> = sub_str.split("\"").collect();
                        if parts.len() >= 2 {
                            return Some(parts[1].to_string());
                        }
                    }
                }
            }
        },
    }
    None
}

fn generate_name_from_url(url: &str) -> String {
    url.chars()
        .filter(|ch| ch.is_alphanumeric())
        .collect::<String>()
        + ".ovpn"
}
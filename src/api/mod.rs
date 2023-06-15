use serde::de::DeserializeOwned;
pub mod result;
pub mod seasons;

// const BASE_URL: &str = "https://components.ifsc-climbing.org/results-api.php?api=event_full_results&result_url=/api/v1/";
const BASE_URL: &str = "https://ifsc.donsz.nl/";

pub async fn request<T: DeserializeOwned>(url: String) -> Option<T> {
    let full_url = dbg!(format!("{BASE_URL}{url}"));
    let res = reqwest::get(&full_url).await.ok()?.text().await.ok()?;
    let cleaned = clean_api_output(res);

    #[cfg(feature = "desktop")]
    {
        use std::{
            io::Write,
            path::{Path, PathBuf},
        };

        // Write it to the cache for later reference.
        // This should probably become a cargo feature.
        let file_url = if url.is_empty() { "root" } else { &url };
        let mut path = PathBuf::from("cache");
        path.push(file_url);
        path.set_extension("json");
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        let mut f = std::fs::File::create(path).unwrap();
        write!(f, "{}", &cleaned).unwrap();
    }

    serde_json::from_str(&cleaned).map_err(|e| dbg!(e)).ok()
}

/// Remove random fucking PHP warnings from the output
fn clean_api_output(x: String) -> String {
    x.lines().filter(|line| !line.starts_with('<')).collect()
}

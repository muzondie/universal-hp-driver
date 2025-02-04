use std::collections::HashMap;
use once_cell::sync::OnceCell;
use serde::Deserialize;

static DRIVER_CACHE: OnceCell<HashMap<String, DriverEntry>> = OnceCell::new();

#[derive(Clone, Deserialize)]
struct DriverEntry {
    hwid: String,
    version: String,
    url: String,
    checksum: String,
    dependencies: Vec<String>,
}

fn get_driver_info(hwid: &str) -> Option<DriverEntry> {
    DRIVER_CACHE.get()
        .and_then(|cache| cache.get(hwid).cloned())
        .or_else(|| fetch_online_driver(hwid))
}

fn fetch_online_driver(hwid: &str) -> Option<DriverEntry> {
    None
}

fn load_local_cache() -> HashMap<String, DriverEntry> {
    HashMap::new()
}
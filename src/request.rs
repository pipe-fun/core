use dotenv_codegen::dotenv;
use serde::de::DeserializeOwned;
use serde::Serialize;

const API_ROOT: &str = dotenv!("DB_API");

pub fn get<T: DeserializeOwned>(url: &str) -> Vec<T> {
    let url = format!("{}{}", API_ROOT, url);
    match reqwest::blocking::get(&url) {
        Ok(response) => {
            match response.json::<Vec<T>>() {
                Ok(data) => data,
                Err(_) => vec![]
            }
        }
        Err(_) => vec![]
    }
}

pub fn put<T: Serialize>(url: &str, data: &T) {
    let client = reqwest::blocking::ClientBuilder::new().build().unwrap();
    match client.put(url).json(data).send() {
        Ok(_) => (),
        Err(_) => ()
    }
}
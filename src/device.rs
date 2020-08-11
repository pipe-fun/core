#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
    owner: String,
    token: String,
}

impl Device {
    pub fn get_owner_by_token(token: &str) -> String {
        let vec = match reqwest::blocking::get("http://localhost:1122/api/device/read") {
            Ok(response) => response.json::<Vec<Device>>().unwrap(),
            Err(_) => { Vec::new() }
        };

        let mut owner = String::new();
        for d in vec {
            if d.token.eq(token) {
                owner = d.owner;
                break;
            }
        }
        owner
    }
}
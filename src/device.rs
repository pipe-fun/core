use crate::request;

#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
    token: String,
    name: String,
    owner: String
}

impl Device {
    pub fn get_owner_by_token(token: &str) -> String {
        let url = "/device/read";
        let vec: Vec<Device> = request::get(url);

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
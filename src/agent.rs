#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Request {
    pub tty: Option<String>,
    pub action: Action,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Action {
    Login,
    Unlock,
    Sync,
    Decrypt { cipherstring: String },
    // add
    // update
    // remove
    Quit,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Response {
    Ack,
    Error { error: String },
    Decrypt { plaintext: String },
}

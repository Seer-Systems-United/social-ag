use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct ProfilePage {
    #[serde(default)]
    pub messages: Vec<ProfileMessage>,
}

#[derive(Deserialize)]
pub(super) struct ProfileMessage {
    pub data: ProfileData,
}

#[derive(Deserialize)]
pub(super) struct ProfileData {
    pub fid: u64,
    #[serde(rename = "userDataBody")]
    pub body: UserDataBody,
}

#[derive(Deserialize)]
pub(super) struct UserDataBody {
    #[serde(rename = "type")]
    pub kind: String,
    pub value: String,
}

#[derive(Deserialize)]
pub(super) struct CastPage {
    #[serde(default)]
    pub messages: Vec<CastMessage>,
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
}

#[derive(Deserialize)]
pub(super) struct CastMessage {
    pub data: CastData,
    pub hash: String,
}

#[derive(Deserialize)]
pub(super) struct CastData {
    pub timestamp: i64,
    #[serde(rename = "castAddBody")]
    pub body: CastBody,
}

#[derive(Deserialize)]
pub(super) struct CastBody {
    pub text: String,
    #[serde(rename = "parentCastId")]
    pub parent: Option<ParentCast>,
}

#[derive(Deserialize)]
pub(super) struct ParentCast {
    pub hash: String,
}

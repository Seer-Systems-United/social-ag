use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct SyndicationData {
    pub props: Props,
}

#[derive(Debug, Deserialize)]
pub(super) struct Props {
    #[serde(rename = "pageProps")]
    pub page_props: PageProps,
}

#[derive(Debug, Deserialize)]
pub(super) struct PageProps {
    #[serde(rename = "contextProvider")]
    pub context_provider: ContextProvider,
    pub timeline: Timeline,
}

#[derive(Debug, Deserialize)]
pub(super) struct ContextProvider {
    #[serde(rename = "hasResults")]
    pub has_results: bool,
}

#[derive(Debug, Deserialize)]
pub(super) struct Timeline {
    #[serde(default)]
    pub entries: Vec<Entry>,
}

#[derive(Debug, Deserialize)]
pub(super) struct Entry {
    pub content: EntryContent,
}

#[derive(Debug, Deserialize)]
pub(super) struct EntryContent {
    pub tweet: Option<SyndicatedPost>,
}

#[derive(Debug, Deserialize)]
pub(super) struct SyndicatedPost {
    pub id_str: String,
    pub created_at: String,
    pub full_text: Option<String>,
    pub text: Option<String>,
    pub permalink: String,
    pub user: SyndicatedUser,
    pub in_reply_to_status_id_str: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(super) struct SyndicatedUser {
    pub name: String,
    pub screen_name: String,
}

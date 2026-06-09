use scraper::Html;

use crate::{Post, SourceError, User, parse};

use super::{Bandcamp, models::CatalogItem, models::Release};

impl Bandcamp {
    pub(super) fn lookup(&self, identifier: &str) -> Result<Option<User>, SourceError> {
        if !self.matches(identifier) {
            return Ok(None);
        }
        self.catalog().map(|(user, _)| Some(user))
    }

    pub(super) fn posts(&self, identifier: &str, count: usize) -> Result<Vec<Post>, SourceError> {
        if count == 0 {
            return Ok(Vec::new());
        }
        if !self.matches(identifier) {
            return Err(SourceError::NotFound);
        }
        let (user, items) = self.catalog()?;
        items
            .into_iter()
            .take(count)
            .map(|item| self.post(item, &user))
            .collect()
    }

    fn post(&self, item: CatalogItem, user: &User) -> Result<Post, SourceError> {
        let body = self.transport.get_text(item.url, "text/html")?;
        let document = Html::parse_document(&body);
        let release = parse::json_ld::documents(&document)
            .into_iter()
            .find_map(|value| serde_json::from_value::<Release>(value).ok())
            .ok_or_else(|| SourceError::InvalidResponse("missing Bandcamp JSON-LD".into()))?;
        let timestamp = release.timestamp()?;
        Ok(Post {
            id: item.id,
            publisher_user: user.clone(),
            title: Some(release.name),
            content: release.description,
            timestamp,
            url: release.url,
            community: None,
            in_reply_to_id: None,
        })
    }

    fn matches(&self, identifier: &str) -> bool {
        self.user.id == identifier
            || self
                .user
                .username
                .eq_ignore_ascii_case(identifier.trim_start_matches('@'))
    }
}

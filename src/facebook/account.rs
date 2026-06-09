use crate::{SourceError, parse::common::required_identifier, sources::User};

use super::{Facebook, models::Page};

impl Facebook {
    pub(super) fn lookup_page(&self, identifier: &str) -> Result<Option<User>, SourceError> {
        self.require_authentication()?;
        let identifier = required_identifier(identifier)?;
        let mut url = self.endpoint(&[identifier]);
        url.query_pairs_mut()
            .append_pair("fields", "id,name,link,username");
        let page: Page = match self.transport.get_json(url) {
            Ok(page) => page,
            Err(SourceError::NotFound) => return Ok(None),
            Err(error) => return Err(error),
        };
        Ok(Some(User {
            id: page.id.clone(),
            username: page.username.unwrap_or(page.id),
            display_name: Some(page.name),
            profile_url: page
                .link
                .unwrap_or_else(|| format!("https://www.facebook.com/{identifier}")),
        }))
    }
}

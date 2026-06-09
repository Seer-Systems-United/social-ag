use scraper::{Html, Selector};

use crate::{SourceError, User, parse::common::optional_text};

use super::{Bandcamp, models::CatalogItem};

impl Bandcamp {
    pub(super) fn catalog(&self) -> Result<(User, Vec<CatalogItem>), SourceError> {
        let body = self
            .transport
            .get_text(self.artist_url.clone(), "text/html")?;
        let document = Html::parse_document(&body);
        let mut user = self.user.clone();
        let name = Selector::parse("#band-name-location .title").unwrap();
        user.display_name = document
            .select(&name)
            .next()
            .and_then(|element| optional_text(element.text().collect::<String>()));
        let item = Selector::parse("li.music-grid-item[data-item-id]").unwrap();
        let link = Selector::parse("a[href]").unwrap();
        let items = document
            .select(&item)
            .filter_map(|element| {
                let anchor = element.select(&link).next()?;
                Some(CatalogItem {
                    id: element.value().attr("data-item-id")?.to_string(),
                    url: self.artist_url.join(anchor.value().attr("href")?).ok()?,
                })
            })
            .collect();
        Ok((user, items))
    }
}

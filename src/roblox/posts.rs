use reqwest::Url;

use crate::{
    Post, SourceError, User,
    parse::common::{optional_text, parse_datetime},
};

use super::{
    Roblox,
    models::{Game, GamePage},
};

impl Roblox {
    pub(super) fn posts(&self, identifier: &str, count: usize) -> Result<Vec<Post>, SourceError> {
        if count == 0 {
            return Ok(Vec::new());
        }
        let Some(user) = self.lookup(identifier)? else {
            return Err(SourceError::NotFound);
        };
        let mut cursor = None;
        let mut posts = Vec::with_capacity(count);
        while posts.len() < count {
            let page: GamePage = self
                .transport
                .get_json(self.games_page(cursor.as_deref(), count))?;
            posts.extend(
                page.data
                    .into_iter()
                    .map(|game| post_from_game(game, &user))
                    .collect::<Result<Vec<_>, _>>()?,
            );
            let Some(next) = page.next_cursor else {
                break;
            };
            cursor = Some(next);
        }
        posts.truncate(count);
        Ok(posts)
    }

    fn games_page(&self, cursor: Option<&str>, count: usize) -> Url {
        let mut url = self.games_url.clone();
        url.path_segments_mut()
            .unwrap()
            .pop_if_empty()
            .extend([self.user_id.as_str(), "games"]);
        let limit = if count <= 10 {
            "10"
        } else if count <= 25 {
            "25"
        } else {
            "50"
        };
        let mut query = url.query_pairs_mut();
        query
            .append_pair("accessFilter", "Public")
            .append_pair("limit", limit)
            .append_pair("sortOrder", "Desc");
        if let Some(cursor) = cursor {
            query.append_pair("cursor", cursor);
        }
        drop(query);
        url
    }
}

fn post_from_game(game: Game, user: &User) -> Result<Post, SourceError> {
    Ok(Post {
        id: game.id.to_string(),
        publisher_user: user.clone(),
        title: optional_text(game.name),
        content: optional_text(game.description),
        timestamp: parse_datetime(&game.updated)?,
        url: format!("https://www.roblox.com/games/{}", game.root_place.id),
        community: None,
        in_reply_to_id: None,
    })
}

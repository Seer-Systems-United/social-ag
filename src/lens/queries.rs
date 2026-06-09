pub(super) const ACCOUNT: &str = r#"
query Account($username: String!) {
  account(request: { username: { localName: $username } }) {
    address
    username { localName }
    metadata { name }
  }
}"#;

pub(super) const POSTS: &str = r#"
query Posts($address: EvmAddress!, $pageSize: PageSize!, $cursor: Cursor) {
  posts(request: {
    filter: { authors: [$address], postTypes: [ROOT] }
    pageSize: $pageSize
    cursor: $cursor
  }) {
    items {
      ... on Post {
        id
        slug
        timestamp
        metadata {
          ... on TextOnlyMetadata { content }
          ... on ArticleMetadata { title content }
          ... on ImageMetadata { title content }
          ... on VideoMetadata { title content }
          ... on AudioMetadata { title content }
          ... on LinkMetadata { content }
        }
      }
    }
    pageInfo { next }
  }
}"#;

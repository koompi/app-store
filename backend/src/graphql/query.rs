use async_graphql::{Context, Object};
pub struct RootQuery;

#[Object]
impl RootQuery {
    pub async fn greet<'a>(&self, ctx: &'a Context<'_>) -> String {
        String::from("Hello")
    }
}

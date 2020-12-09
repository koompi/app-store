use async_graphql::{Context, Object};
pub struct RootQuery;

#[Object]
impl RootQuery {
    pub async fn greet(&self, _ctx: &Context<'_>) -> String {
        String::from("Hello")
    }
}

use async_graphql::{Context, Object};

pub struct RootMutation;

#[Object]
impl RootMutation {
    pub async fn say<'a>(&self, ctx: &'a Context<'_>) -> String {
        String::from("Say something")
    }
}

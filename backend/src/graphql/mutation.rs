// Library imports
use async_graphql::{Context, Object};

pub struct RootMutation;

#[Object]
impl RootMutation {
    pub async fn say(&self, _ctx: &Context<'_>) -> String {
        String::from("Say something")
    }
}

// Library imports
use actix_web::{web, HttpRequest, HttpResponse};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{Request, Response};
use mongodb::Client;

// Local imports
use crate::graphql::{AppContext, MainSchema, Token};
pub async fn index(
    pool: web::Data<Client>,
    schema: web::Data<MainSchema>,
    req: HttpRequest,
    gql_request: Request,
) -> Response {
    // Get request token from header
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().map(|s| Token(s.to_string())).ok());

    // Create GraphQL context and pass db connection poll into
    let context = AppContext {
        db_pool: pool.get_ref().to_owned(),
    };

    let mut request = gql_request.into_inner();
    // inject token of availble
    if let Some(token) = token {
        request = request.data(token);
    }
    // inject db pool to context
    request = request.data(context);
    schema.execute(request).await.into()
}

pub async fn gql_playgound() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/api").subscription_endpoint("/api"),
        ))
}

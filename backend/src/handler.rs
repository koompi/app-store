use crate::graphql::root::{MainSchema, Token};
use actix_web::{web, HttpRequest, HttpResponse};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{Request, Response};

pub async fn index(
    schema: web::Data<MainSchema>,
    req: HttpRequest,
    gql_request: Request,
) -> Response {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().map(|s| Token(s.to_string())).ok());
    let mut request = gql_request.into_inner();
    if let Some(token) = token {
        request = request.data(token);
    }
    schema.execute(request).await.into()
}

pub async fn gql_playgound() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        ))
}

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::Extension;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;
use hyper::Server;
use tower_http::cors::CorsLayer;

use rust_axum_async_graphql_blog::graphql::model::{
    GraphQLSchema, MutationRoot, QueryRoot, Storage,
};

async fn graphql_handler(schema: Extension<GraphQLSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.0).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[tokio::main]
async fn main() {
    let _ = pretty_env_logger::try_init();

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(Storage::new("./posts"))
        .finish();

    println!("Playground: http://localhost:8000");

    let app = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .layer(Extension(schema))
        .layer(CorsLayer::permissive());

    Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

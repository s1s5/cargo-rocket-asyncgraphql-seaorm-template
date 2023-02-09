mod graphql;
pub use graphql::export_sdl;

extern crate rocket;
use async_graphql::http::GraphiQLSource;
use async_graphql_rocket::{GraphQLQuery, GraphQLRequest, GraphQLResponse};
use entity::db::Database;
use rocket::{
    request::{self, FromRequest, Outcome},
    response::content,
    routes, Config, Request, State,
};
use std::{
    convert::Infallible,
    net::{IpAddr, Ipv4Addr},
};

#[derive(Debug)]
pub enum AuthorizationToken {
    None,
    Bearer(String),
    Unknown(String),
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthorizationToken {
    type Error = Infallible;

    // async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error>;
    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token = request.headers().get_one("authorization");
        match token {
            Some(token) => {
                let v: Vec<_> = token.split(" ").collect();
                if v.len() == 2 && v[0].to_lowercase() == "bearer" {
                    Outcome::Success(AuthorizationToken::Bearer(v[1].into()))
                } else {
                    Outcome::Success(AuthorizationToken::Unknown(token.to_string()))
                }
            }
            None => Outcome::Success(AuthorizationToken::None),
        }
    }
}

#[rocket::get("/")]
fn graphiql() -> content::RawHtml<String> {
    content::RawHtml(GraphiQLSource::build().endpoint("/graphql").finish())
}

#[rocket::get("/graphql?<query..>")]
async fn graphql_query(
    schema: &State<graphql::AppSchema>,
    query: GraphQLQuery,
    auth_token: AuthorizationToken,
) -> GraphQLResponse {
    let request: GraphQLRequest = query.into();
    request.data(auth_token).execute(schema.inner()).await
}

#[rocket::post("/graphql", data = "<request>", format = "application/json")]
async fn graphql_request(
    schema: &State<graphql::AppSchema>,
    request: GraphQLRequest,
    auth_token: AuthorizationToken,
) -> GraphQLResponse {
    request.data(auth_token).execute(schema.inner()).await
}

#[tokio::main]
async fn start() -> Result<(), rocket::Error> {
    let db = Database::new().await;

    let schema = graphql::build()
        // .data(db)
        .data(async_graphql::dataloader::DataLoader::new(
            db,
            tokio::task::spawn,
        ))
        .enable_federation()
        .extension(async_graphql::extensions::Logger)
        .finish();

    rocket::build()
        .manage(schema)
        .mount("/", routes![graphql_query, graphql_request, graphiql])
        .configure(Config {
            address: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            port: 8000,
            ..Default::default()
        })
        .launch()
        .await
        .map(|_| ())
}

pub fn main() {
    let result = start();

    println!("Rocket: deorbit.");

    if let Some(err) = result.err() {
        println!("Error: {}", err);
    }
}

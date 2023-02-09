use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Object, SDLExportOptions, Schema};


#[derive(Default)]
struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self) -> Result<&'static str> {
        Ok("hello")
    }
}

#[derive(MergedObject, Default)]
pub struct Query(QueryRoot);

// #[derive(MergedObject, Default)]
// pub struct Mutation(MutationRoot);
type Mutation = EmptyMutation;

pub type Result<T> = std::result::Result<T, async_graphql::Error>;
pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn build() -> async_graphql::SchemaBuilder<Query, Mutation, EmptySubscription> {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
}

pub fn export_sdl() -> String {
    let schema = build().enable_federation().finish();
    schema.sdl_with_options(SDLExportOptions::new().federation())
}

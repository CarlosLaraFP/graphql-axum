use async_graphql::{Context, Object, Schema};
use async_graphql::{EmptyMutation, EmptySubscription};


pub(crate) type MatchingSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;
pub(crate) struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn get_match(&self, _ctx: &Context<'_>, worker_id: u32) -> String {
        format!("Worker ID {} matched to General Helper job", worker_id)
    }
}

// curl -X POST -H "Content-Type: application/json" --data '{ "query": "{ getMatch(workerId: 1) }" }' http://localhost:8000
use crate::server_modules::model::interesting_fact::*;
use sqlx::postgres::PgPool;
use std::error::Error;

// get all interesting facts  records from DB   
pub async fn get_interesting_facts_db(
  pool: &PgPool,
) -> Vec<Fact> {
  sqlx::query_as!(
      Fact, 
      "SELECT * FROM interesting_fact"
      )
      .fetch_all(pool)
      .await
      .unwrap()
}
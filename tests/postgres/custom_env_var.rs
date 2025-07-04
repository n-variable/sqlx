#[sqlx_macros::test(db_env_var = "MY_DB_URL", migrations = false)]
async fn random_macro(_pool: sqlx::Pool<sqlx::Postgres>) -> anyhow::Result<()> {
    println!("pool: {:?}", _pool);
   Ok(())
}
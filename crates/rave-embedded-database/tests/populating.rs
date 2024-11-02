use rave_embedded_database::prelude::*;
use tracing_test::traced_test;

#[tokio::test]
#[traced_test]
async fn populate_database() {
    let pool = EmbeddedDatabasePool::new().await.unwrap();
    let db1 = pool.create_database(None).await.unwrap();
    db1.run_migrations().await.unwrap();
}

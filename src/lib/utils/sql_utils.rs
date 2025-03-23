use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, MySqlConnection, QueryBuilder, Connection};
// etc.
struct SecureNote {
    title: String,
    content: String,
    password: String,
    NoteId: i32,
}
const BIND_LIMIT: usize = 65535;

struct MyMySql<'a> {
    query_builder: QueryBuilder<'a, MySql>,
    connection: MySqlConnection,

}

impl MyMySql<'_> {
    pub async fn new() -> anyhow::Result<Self> {
        Ok(Self {
            query_builder: QueryBuilder::new(
                // Note the trailing space; most calls to `QueryBuilder` don't automatically insert
                // spaces as that might interfere with identifiers or quoted strings where exact
                // values may matter.
                "SELECT title FROM SecureNote WHERE NoteId = ?1",
            ),
            connection: {
                let connection = MySqlConnection::connect("sqlite::memory:").await?;

                connection
            },
        })
    }
}

//#[tokio::main]
//pub async fn foobar() -> Result<(), sqlx::Error> {
pub async fn foobar() -> anyhow::Result<()> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("sqlite::memory:")
        .await?;

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL/MariaDB)
    let row: (i64,) = sqlx::query_as("SELECT ?1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    assert_eq!(row.0, 150);

    Ok(())
}

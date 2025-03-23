use gavlib::utils::sql_utils::{new_sql_connection, SecureNote};
use sqlx::{Connection, MySqlConnection};

pub struct GavDaemon {
    sql_connection: MySqlConnection,
}

impl GavDaemon {
    pub async fn new() -> anyhow::Result<Self> {
        Ok(Self {
            sql_connection: new_sql_connection().await?,
        })
    }
    pub fn search(&self, input:String) -> Vec<SecureNote>{
        
    }

}

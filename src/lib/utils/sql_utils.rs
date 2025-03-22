use mysql::{
    PooledConn, Pool, params,
    prelude::{Queryable},
};

type DbResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct SqlConnection {
    conn: PooledConn,
}

struct UserDb {
    username: String,
    password: String,
    website: String
}

impl SqlConnection {
    pub fn new(url: &str) -> DbResult<Self> {
        Ok(Self {
            conn: Pool::new(url)?.get_conn()?
        })
    }

// Stores the password
pub fn store_password(&mut self) -> DbResult<()> {
    //Sample insert w/o implementation on UI as we will do that later.
    
    let added_accounts = vec! [
        UserDb{username: "test1".to_string(), password: "test1".to_string(), website: "test1.com".to_string()}
    ];

    Ok(self.conn.exec_batch(
        r"INSERT INTO user_db (username, password, website)
          VALUES (:username, :password, :website)",
          added_accounts.iter().map(|p| params! {
            "username" => &p.username,
            "password" => &p.password,
            "website" => &p.website,
        })
    )?)
}

// Retrive password
pub fn retrieve_password(){
    //Sample select w/o implementation on UI as we will do that later.
    let selected_accounts = conn.query_map(
        "SELECT username, password, website FROM user_db",
        |(username, password, website)| {
            UserDb {
                username,
                password,
                website
            }
        },
    )?;
    assert_eq!(added_accounts, selected_accounts);
}
}

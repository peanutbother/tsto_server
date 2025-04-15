use crate::{
    config::OPTIONS,
    util::{relative_path, DIRECTORIES},
};
use axum::Extension;
use futures::{future::BoxFuture, stream::BoxStream};
use once_cell::sync::OnceCell;
use rand::{distr::Alphanumeric, Rng};
use sqlx::{migrate::MigrateDatabase, Pool, Sqlite};
use std::fs::create_dir_all;
use tower_sessions_sqlx_store::SqliteStore;
use tracing::{error, info};

pub static DATABASE: OnceCell<Database> = OnceCell::new();

pub async fn init() -> anyhow::Result<()> {
    let db = Database::new(&{
        let database = OPTIONS.take().database.clone();
        let is_portable = OPTIONS.take().portable;

        if &database == ":memory:" {
            database
        } else {
            let mut path = if is_portable {
                relative_path()?
            } else {
                DIRECTORIES.data_local_dir().to_path_buf()
            };
            if !path.exists() {
                create_dir_all(&path).expect("directory is writable");
            }
            path.push(&database);
            path.to_str().expect("path is valid utf-8").to_owned()
        }
    })
    .await?;

    DATABASE.set(db).expect("database was not initialized yet");

    Ok(())
}

#[derive(Debug, Clone)]
pub struct Database {
    pool: Pool<Sqlite>,
    session_store: SqliteStore,
}

impl Database {
    async fn new(path: &str) -> anyhow::Result<Self> {
        if !Sqlite::database_exists(path).await.unwrap_or(false) {
            info!("creating database at {}", path);
            match Sqlite::create_database(path).await {
                Ok(_) => info!("successfully created database"),
                Err(error) => error!("error: {}", error),
            }
        }

        info!("connecting to database at {}", path);
        let pool = sqlx::sqlite::SqlitePool::connect(path).await?;
        let session_store = SqliteStore::new(pool.clone())
            .with_table_name("sessions")
            .map_err(|e| anyhow::anyhow!(e))?;

        info!("running migrations");
        sqlx::migrate!().run(&pool).await?;
        session_store.migrate().await?;

        if !check_auth_has_owner(&pool).await? {
            let password = generate_admin_owner(&pool).await?;
            info!(
                r#"generated initial user "admin" with password: "{}""#,
                password
            );
        }

        Ok(Self {
            pool,
            session_store,
        })
    }

    pub async fn extension() -> anyhow::Result<Extension<Self>> {
        Ok(Extension(
            DATABASE
                .get()
                .ok_or(anyhow::anyhow!("use of db before initialization"))?
                .clone(),
        ))
    }

    pub fn session_store() -> anyhow::Result<SqliteStore> {
        Ok(DATABASE
            .get()
            .ok_or(anyhow::anyhow!("use of db before initialization"))?
            .clone()
            .session_store)
    }
}

impl<'c> sqlx::Executor<'c> for &Database {
    type Database = Sqlite;

    fn fetch_many<'e, 'q: 'e, E>(
        self,
        query: E,
    ) -> BoxStream<
        'e,
        Result<
            sqlx::Either<
                <Self::Database as sqlx::Database>::QueryResult,
                <Self::Database as sqlx::Database>::Row,
            >,
            sqlx::Error,
        >,
    >
    where
        'c: 'e,
        E: 'q + sqlx::Execute<'q, Self::Database>,
    {
        self.pool.fetch_many(query)
    }

    fn fetch_optional<'e, 'q: 'e, E>(
        self,
        query: E,
    ) -> BoxFuture<'e, Result<Option<<Self::Database as sqlx::Database>::Row>, sqlx::Error>>
    where
        'c: 'e,
        E: 'q + sqlx::Execute<'q, Self::Database>,
    {
        self.pool.fetch_optional(query)
    }

    fn prepare_with<'e, 'q: 'e>(
        self,
        sql: &'q str,
        parameters: &'e [<Self::Database as sqlx::Database>::TypeInfo],
    ) -> BoxFuture<'e, Result<<Self::Database as sqlx::Database>::Statement<'q>, sqlx::Error>>
    where
        'c: 'e,
    {
        self.pool.prepare_with(sql, parameters)
    }

    fn describe<'e, 'q: 'e>(
        self,
        sql: &'q str,
    ) -> BoxFuture<'e, Result<sqlx::Describe<Self::Database>, sqlx::Error>>
    where
        'c: 'e,
    {
        self.pool.describe(sql)
    }
}

async fn check_auth_has_owner(pool: &Pool<Sqlite>) -> Result<bool, sqlx::Error> {
    const QUERY_OWNER: &str = r#"SELECT COUNT() FROM auth WHERE role = "owner""#;
    Ok(sqlx::query_scalar::<_, i32>(QUERY_OWNER)
        .fetch_one(pool)
        .await?
        > 0)
}

async fn generate_admin_owner(pool: &Pool<Sqlite>) -> Result<String, sqlx::Error> {
    const CREATE_OWNER: &str = r#"INSERT INTO auth 
    (username, password, role)
    VALUES("admin", ?, "owner")"#;

    let rng = rand::rng();
    let password: String = rng
        .sample_iter(Alphanumeric)
        .take(8)
        .map(|i| i as char)
        .collect();

    sqlx::query(CREATE_OWNER)
        .bind(password_auth::generate_hash(password.clone()))
        .execute(pool)
        .await?;

    Ok(password)
}

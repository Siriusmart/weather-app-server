use std::{error::Error, fs, ops::Deref, path::Path, sync::OnceLock};

use diesel::{
    RunQueryDsl, SqliteConnection,
    r2d2::{ConnectionManager, CustomizeConnection, Pool, PooledConnection},
};

static DATABASE: OnceLock<Database> = OnceLock::new();

pub type DatabaseBackend = SqliteConnection;
pub type DatabasePool = Pool<ConnectionManager<DatabaseBackend>>;
pub type DatabaseConnection = PooledConnection<ConnectionManager<DatabaseBackend>>;

/// connection to the sqlite database
#[derive(Clone)]
pub struct Database(DatabasePool);

impl Database {
    pub fn init(path: &Path) -> Result<(), Box<dyn Error>> {
        let _ = DATABASE.set(Self::open(path)?);
        Ok(())
    }

    /// open a connection
    fn open(path: &Path) -> Result<Self, Box<dyn Error>> {
        if !path.parent().unwrap().exists() {
            fs::create_dir_all(path.parent().unwrap())?;
        }

        let manager = ConnectionManager::<DatabaseBackend>::new(path.to_string_lossy());

        Ok(Self(
            Pool::builder()
                .connection_customizer(Box::new(ForeignKeyEnforcer)) // foreign key checks in sqlite
                .idle_timeout(None) // hopefully disables timeout
                .build(manager)
                .unwrap(),
        ))
    }

    pub fn conn() -> DatabaseConnection {
        DATABASE
            .get()
            .unwrap()
            .0
            .get()
            .expect("shouldn't fail if there is no timeout")
    }
}

/// for transprent wrapper struct
impl Deref for Database {
    type Target = DatabasePool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
struct ForeignKeyEnforcer;

impl CustomizeConnection<DatabaseBackend, diesel::r2d2::Error> for ForeignKeyEnforcer {
    fn on_acquire(&self, conn: &mut DatabaseBackend) -> Result<(), diesel::r2d2::Error> {
        diesel::sql_query("PRAGMA foreign_keys = ON")
            .execute(conn)
            .map_err(diesel::r2d2::Error::QueryError)?;

        Ok(())
    }
}

use std::path::PathBuf;
use tempfile::TempDir;

/// A temporary test database that is cleaned up on drop.
pub struct TestDb {
    #[allow(dead_code)]
    dir: TempDir,
    path: PathBuf,
}

impl TestDb {
    /// Create a new temporary SQLite database.
    pub fn new(name: &str) -> Self {
        let dir = TempDir::new().expect("failed to create temp dir");
        let path = dir.path().join(format!("{name}.db"));
        Self { dir, path }
    }

    /// Get the path to the database file.
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Get the connection string for SQLite.
    pub fn connection_string(&self) -> String {
        format!("sqlite:{}", self.path.display())
    }

    /// Initialize the database with a schema SQL string.
    pub fn init_schema(&self, schema: &str) -> Result<(), Box<dyn std::error::Error>> {
        let conn = rusqlite::Connection::open(&self.path)?;
        conn.execute_batch(schema)?;
        Ok(())
    }
}

impl Drop for TestDb {
    fn drop(&mut self) {
        // TempDir cleanup handles the file
    }
}

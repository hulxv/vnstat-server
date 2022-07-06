use anyhow::Result;
use diesel::SqliteConnection;

/// Create or insert new values.
pub trait Create {
    type Output;
    fn create(&self, conn: &SqliteConnection) -> Result<Self::Output>;
}

pub trait Statements {
    type Args;
    type SelectOutput;
    type FindOutput;

    fn select<F>(conn: &SqliteConnection, f: F) -> Self::SelectOutput
    where
        F: Fn(Self::Args) -> bool;
    fn find<F>(conn: &SqliteConnection, f: F) -> Self::FindOutput
    where
        F: Fn(Self::Args) -> bool;
}

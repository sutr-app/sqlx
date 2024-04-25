use std::iter::{Extend, IntoIterator};
use sqlx_core::query_result::{HasLastInsertId, HasRowsAffected};

#[derive(Debug, Default)]
pub struct SqliteQueryResult {
    pub(super) changes: u64,
    pub(super) last_insert_rowid: i64,
}

impl HasLastInsertId<i64> for SqliteQueryResult {
    fn last_insert_id(&self) -> i64 {
        self.last_insert_rowid
    }
}

impl HasRowsAffected for SqliteQueryResult {
    fn rows_affected(&self) -> u64 {
        self.changes
    }
}

impl Extend<SqliteQueryResult> for SqliteQueryResult {
    fn extend<T: IntoIterator<Item = SqliteQueryResult>>(&mut self, iter: T) {
        for elem in iter {
            self.changes += elem.changes;
            self.last_insert_rowid = elem.last_insert_rowid;
        }
    }
}

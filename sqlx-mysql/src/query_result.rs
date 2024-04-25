use std::iter::{Extend, IntoIterator};
use sqlx_core::query_result::{HasLastInsertId, HasRowsAffected};

#[derive(Debug, Default)]
pub struct MySqlQueryResult {
    pub(super) rows_affected: u64,
    pub(super) last_insert_id: u64,
}

impl HasRowsAffected for MySqlQueryResult {
    fn rows_affected(&self) -> u64 {
        self.rows_affected
    }
}

impl HasLastInsertId<u64> for MySqlQueryResult {
    fn last_insert_id(&self) -> u64 {
        self.last_insert_id
    }
}

impl Extend<MySqlQueryResult> for MySqlQueryResult {
    fn extend<T: IntoIterator<Item = MySqlQueryResult>>(&mut self, iter: T) {
        for elem in iter {
            self.rows_affected += elem.rows_affected;
            self.last_insert_id = elem.last_insert_id;
        }
    }
}

use std::iter::{Extend, IntoIterator};
use sqlx_core::query_result::{HasLastInsertId, HasRowsAffected};

#[derive(Debug, Default)]
pub struct PgQueryResult {
    pub(super) rows_affected: u64,
}

impl HasLastInsertId<Option<()>> for PgQueryResult {
    fn last_insert_id(&self) -> Option<()> {
        None
    }
}

impl HasRowsAffected for PgQueryResult {
    fn rows_affected(&self) -> u64 {
        self.rows_affected
    }
}

impl Extend<PgQueryResult> for PgQueryResult {
    fn extend<T: IntoIterator<Item = PgQueryResult>>(&mut self, iter: T) {
        for elem in iter {
            self.rows_affected += elem.rows_affected;
        }
    }
}

#[cfg(feature = "any")]
impl From<PgQueryResult> for crate::any::AnyQueryResult {
    fn from(done: PgQueryResult) -> Self {
        crate::any::AnyQueryResult {
            rows_affected: done.rows_affected,
            last_insert_id: None,
        }
    }
}

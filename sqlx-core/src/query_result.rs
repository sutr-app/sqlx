
pub trait HasRowsAffected {
    fn rows_affected(&self) -> u64;
}

pub trait HasLastInsertId<T> {
    fn last_insert_id(&self) -> T;
}

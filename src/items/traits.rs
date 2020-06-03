// This is an attempt on making an API, but Diesel is horrible to use generically.
use diesel::asssociations;

/// A trait representing the table for a type in the database.
trait Table {
    /// The table itself
    type Table: associations::HasTable;
}

trait Builder {
    type Output;

    fn new() -> Self;

    fn set_owner_id(&mut self, uuid: Uuid) -> Self;
    
    fn build(self) -> Self::Output;

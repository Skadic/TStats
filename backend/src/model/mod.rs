pub mod tournament;
pub mod stage;
pub mod map;

pub trait TableType {
    fn table_name() -> &'static str;
}

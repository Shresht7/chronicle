use crate::models::Table;

pub trait OutputFormatter {
    fn format(&self, table: &Table) -> String;
}

mod tsv;
pub use tsv::TsvFormatter;

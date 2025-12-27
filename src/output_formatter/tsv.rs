use super::OutputFormatter;
use crate::models::Table;

pub struct TsvFormatter;

impl OutputFormatter for TsvFormatter {
    fn format(&self, table: &Table) -> String {
        let mut output = String::new();
        output.push_str(&table.headers.join("\t"));
        output.push('\n');
        for row in &table.rows {
            output.push_str(&row.join("\t"));
            output.push('\n');
        }
        output
    }
}

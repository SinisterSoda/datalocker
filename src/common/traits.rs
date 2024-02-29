use mysql::Row;

use crate::lockers::builders::select::SelectBuilder;


pub trait BuildsQueries {
    fn new() -> Self;
    fn create(&self, table: &str, fields: &[(&str, &str)]) -> String;
    fn insert<T: QueryData>(&self, table: &str, data: &[T])  -> String;
    fn select_raw(&self, table: &str, cols: &str, where_clause: Option<&str>, order_by: Option<&str>, limit: Option<&str>) -> String;
    fn select(&self, select_obj: &SelectBuilder) -> String;
}

pub trait QueryData {
    fn to_query_string(&self) -> String;
    fn to_column_string(&self) -> String {
        self.to_column_array().join(", ")
    }
    fn to_column_array(&self) -> &[&str];
}

pub trait FromLockerRow {
    fn from_row(row: Row) -> Self;
}
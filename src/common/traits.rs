use mysql::Row;



pub trait BuildsQueries {
    fn new() -> Self;
    fn create(&self, table: &str, fields: &[(&str, &str)]) -> String;
    fn insert<T: QueryData>(&self, table: &str, data: &[T])  -> String;
    fn select_raw(&self, table: &str, cols: &str, where_clause: Option<&str>, order_by: Option<&str>, limit: Option<&str>) -> String;
    fn select<T: BuildsClauses>(&self, select_obj: &T) -> String;
    fn delete_raw(&self, table: &str, where_clause: Option<&str>) -> String;
    fn delete<T: BuildsClauses>(&self, select_obj: &T) -> String;
    fn update_raw(&self, table: &str, where_clause: Option<&str>, field_updates: &[(&str, &str)], ) -> String;
    fn update<T: BuildsClauses>(&self, clause_obj: &T, field_updates: &[(&str, &str)]) -> String;
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

pub trait BuildsClauses {
    fn add_where(self, clause: &str) -> Self;
    fn add_where_subquery<T: BuildsClauses>(self, clause: &str, sb: T) -> Self;
    fn and(self) -> Self;
    fn or(self) -> Self;
    fn build(&self) -> String;
    fn get_table(&self) -> String;
}

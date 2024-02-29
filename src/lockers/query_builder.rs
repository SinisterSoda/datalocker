use crate::common::enums::DB;
use crate::common::traits::{BuildsQueries, QueryData};

use super::builders::select::SelectBuilder;



pub struct QueryBuilder<T: BuildsQueries> {
    db: DB,
    _builder: T
}




impl<T: BuildsQueries> QueryBuilder<T> {
    pub fn new(db_type: DB) -> Self {
        Self {
            db: db_type,
            _builder: T::new()
        }
    }

    pub fn create(&mut self, table: &str, fields: &[(&str, &str)]) -> String {
        self._builder.create(table, fields)
    }

    pub fn insert<Q: QueryData>(&self, table: &str,  data: &[Q]) -> String {
        self._builder.insert(table, data)
    }

    pub fn select_raw(&self, table: &str, cols: &str, where_clause: Option<&str>, order_by: Option<&str>, limit: Option<&str>) -> String {
        self._builder.select_raw(table, cols, where_clause, order_by, limit)
    }

    pub fn select(&self, selectObj: &SelectBuilder) -> String{
        self._builder.select(selectObj)
    }


}



use crate::common::traits::{BuildsQueries, QueryData};

use super::{delete::DeleteBuilder, select::SelectBuilder};



pub struct MySqlBuilder {
    _macro_list_create: &'static[(&'static str, &'static str)],
    _macro_list_select: &'static[(&'static str, &'static str)],
    _macro_list_insert: &'static[(&'static str, &'static str)]
}

impl BuildsQueries for MySqlBuilder {
    fn create(&self, table: &str, fields: &[(&str, &str)]) -> String {
        let mut q = format!("CREATE TABLE {table} (");

        let mut it = fields.iter().peekable();
        while let Some(field) = it.next() {
            let sq = format!("{} {}", field.0, field.1);
            q += &sq;
            if !it.peek().is_none() {
                q += &", ";
            }
            else {
                q += &")";
            }
        }



        String::from(q)
    }

    fn new() -> Self {
        Self {
            _macro_list_create: &[
                ("<primary id>", "int not null PRIMARY KEY AUTO_INCREMENT")
            ],
            _macro_list_select: &[
                
            ],
            _macro_list_insert: &[

            ]
        }
    }

    fn insert<T: QueryData>(&self, table: &str,  data: &[T]) -> String {
        let mut q: String = format!("INSERT INTO {table} ");
        if data.len() > 0{
            let cols = &data[0].to_column_array();
            let joined = cols.join(", ");
            q += format!("({}) VALUES ", joined).as_str();

            for d in data {
                q += format!("({}), ", d.to_query_string()).as_str();
            }
            //remove last ', ' 
            q.pop();
            q.pop();
            
        }
        

        q
    }

    fn select(&self, select_obj: &SelectBuilder) -> String{
        select_obj.build()
    }

    fn select_raw(&self, table: &str, cols: &str, where_clause: Option<&str>, order_by: Option<&str>, limit: Option<&str>) -> String {
        let mut q = format!("SELECT {} FROM {}", cols, table);
        match where_clause {
            Some(wc) => {
                q = q + " WHERE " + wc;
            }
            None => {
                
            }
        };

        match order_by {
            Some(ob) => {
                q = q + " " + ob
            }
            None => {

            }
        }

        match limit {
            Some(l) => {
                q = q + " " + l
            }
            None => {

            }
        }


        q
    }
    fn delete(&self, select_obj: &DeleteBuilder) -> String{
        select_obj.build()
    }

    fn delete_raw(&self, table: &str, where_clause: Option<&str>) -> String {
        let mut q = format!("DELETE FROM {}", table);
        match where_clause {
            Some(wc) => {
                q = q + " WHERE " + wc;
            }
            None => {
                
            }
        }

        q
    }

}



use crate::common::traits::{BuildsClauses, BuildsQueries, QueryData};




pub struct MySqlBuilder {

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

    fn select<T: BuildsClauses>(&self, select_obj: &T) -> String{
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
    fn delete<T: BuildsClauses>(&self, select_obj: &T) -> String{
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
    
    fn update_raw(&self, table: &str, where_clause: Option<&str>, fields: &[(&str, &str)]) -> String {
        let mut sets = String::from("SET ");

        let mut it = fields.iter().peekable();

        while let Some(field) = it.next() {
            let sq = format!("{} = {}", field.0, field.1);
            sets += &sq;
            if !it.peek().is_none() {
                sets += &", ";
            }
        }

        let mut wheres = String::from("");
        match where_clause {
            None => {

            },
            Some(w) => {
                wheres = format!(" WHERE {}", w);
            }
        };
        
        let q = format!("UPDATE {} {}{}", table, sets, wheres);

        q
    }
    
    fn update<T: BuildsClauses>(&self, clause_obj: &T, field_updates: &[(&str, &str)]) -> String {
        let mut sets = String::from("SET ");

        let mut it = field_updates.iter().peekable();

        while let Some(field) = it.next() {
            let sq = format!("{} = {}", field.0, field.1);
            sets += &sq;
            if !it.peek().is_none() {
                sets += &", ";
            }
        }

        let q = format!("UPDATE {} {} {}", clause_obj.get_table(), sets, clause_obj.build());
        q
    }

}


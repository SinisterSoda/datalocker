
use crate::common::traits::{BuildsQueries, QueryData};
use crate::common::enums::MacroType;

use super::select::SelectBuilder;



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
            let sq = format!("{} {}", field.0, self.apply_macros(field.1, MacroType::Create));
            q += &sq;
            if (!it.peek().is_none()) {
                q += &", ";
            }
            else {
                q += &")";
            }
        }



        return String::from(q);
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

    fn apply_macros(&self, string: &str, t: MacroType) -> String {
        //let s = self._macro_primary_key(&string);
        let newstring = match t {
            MacroType::Create => {
                let mut s = string.to_string();
                for m in self._macro_list_create {
                    s = s.replace(m.0, m.1)
                }
                s
            }
            MacroType::Insert =>{
                let mut s = string.to_string();
                for m in self._macro_list_insert {
                    s = s.replace(m.0, m.1)
                }
                s
            }
            MacroType::Select => {
                let mut s = string.to_string();
                for m in self._macro_list_select {
                    s = s.replace(m.0, m.1)
                }
                s
            }
        };
        


        newstring.to_string()
    }

    fn insert<T: QueryData>(&self, table: &str,  data: &[T]) -> String {
        let mut q: String = format!("INSERT INTO {table} ");
        if (data.len() > 0){
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

    fn select(&self, selectObj: &SelectBuilder) -> String{
        selectObj.build()
    }

    fn select_raw(&self, table: &str, cols: &str, where_clause: Option<&str>, order_by: Option<&str>, limit: Option<&str>) -> String {
        let mut q = format!("SELECT {} FROM {}", cols, table);
        match where_clause {
            Some(wc) => {
                q = q + " " + wc;
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

}


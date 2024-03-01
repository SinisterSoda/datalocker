use crate::common::{enums::ClauseType, traits::BuildsClauses};




pub struct SelectBuilder {
    where_clauses: Vec<String>,
    order_by: Option<(String, String)>,
    limit: Option<(u32, Option<u32>)>,
    table: String,
    columns: Vec<String>
}

impl BuildsClauses for SelectBuilder {
    

    fn add_where(self, clause: &str) -> Self {
        let mut s = self;
        s.where_clauses.push(clause.to_string());
        s
    }

    fn add_where_subquery<T: BuildsClauses>(self, clause: &str, sb: T) -> Self {
        let mut s: SelectBuilder = self;

        let clause = format!("{} ({})", clause, sb.build());

        s.where_clauses.push(clause);
        s
    }

    fn and(self) -> Self {
        self.add_where("AND")
    }
    fn or(self) -> Self {
        self.add_where("OR")
    }

    

    fn build(&self) -> String {

        let part1 = format!("SELECT {} FROM {}", self.columns.join(", "), self.table);

        let mut wheres: String = String::from("");
        if self.where_clauses.len() > 0{
            wheres = format!(" WHERE {}", self.where_clauses.join(" "))
        }
        let temp_ob = &self.order_by;
        let orders = match temp_ob {
            None => "".to_string(),
            Some(ob) => {
                format!(" ORDER BY {} {}", ob.0, ob.1)
            }
        };
        let temp_limit = self.limit;

        let lim: String = match temp_limit {
            None => { "".to_string() }
            Some(lim) => {
                let l2 = match lim.1 {
                    None => "".to_string(),
                    Some(l) => { ", ".to_owned() + l.to_string().as_str() }
                };

                format!(" LIMIT {}{}", lim.0, l2)
            }
        };

        let part2 = format!("{}{}{}", wheres, orders, lim);

        part1 + &part2



    }
}

impl SelectBuilder {
    pub fn new(table: &str, cols: &[String]) -> Self {
        let mut v: Vec<String> = Vec::new();
        for s in cols {
            v.push(s.to_string());
        }
        Self {
            where_clauses: Vec::new(),
            order_by: None,
            limit: None,
            table: table.to_string(),
            columns: v
        }
    }

    pub fn set_limit(self, l1: u32, l2: Option<u32>) -> Self {
        let mut s = self;
        s.limit = Some((l1, l2));
        s
    }

    pub fn set_order_by(self, ob1: &str, ob2: Option<&str>) -> Self {
        let mut s = self;
        let ob_dir = match ob2 {
            Some(o) => o,
            None => "ASC"
        };
        s.order_by = Some((ob1.to_string(), ob_dir.to_string()));

        s
    }
}


pub struct ClauseBuilder {
    where_clauses: Vec<String>,
    table: String,
    c_type: ClauseType
}

impl BuildsClauses for ClauseBuilder {
    

    fn add_where(self, clause: &str) -> Self {
        let mut s = self;
        s.where_clauses.push(clause.to_string());
        s
    }

    fn add_where_subquery<T: BuildsClauses>(self, clause: &str, sb: T) -> Self {
        let mut s = self;

        let clause = format!("{} ({})", clause, sb.build());

        s.where_clauses.push(clause);
        s
    }

    fn and(self) -> Self {
        self.add_where("AND")
    }
    fn or(self) -> Self {
        self.add_where("OR")
    }

    fn build(&self) -> String {

        let (cmd, tbl) = match self.c_type {
            ClauseType::Delete => ("DELETE FROM ", self.table.as_str()),
            ClauseType::Select => ("SELECT * FROM ", self.table.as_str()),
            ClauseType::Update => ("", "")
        };

        let part1 = format!("{}{}", cmd, tbl);

        let mut wheres: String = String::from("");
        if self.where_clauses.len() > 0{
            wheres = format!(" WHERE {}", self.where_clauses.join(" "))
        }
       

        

        let part2 = format!("{}", wheres);

        part1 + &part2



    }
}

impl ClauseBuilder {
    pub fn new(table: &str, ct: ClauseType) -> Self {
        Self {
            where_clauses: Vec::new(),
            table: table.to_string(),
            c_type: ct
        }
    }
}


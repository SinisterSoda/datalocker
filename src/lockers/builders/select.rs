
pub struct SelectBuilder {
    where_clauses: Vec<String>,
    order_by: Option<(String, String)>,
    limit: Option<(u32, Option<u32>)>,
    table: String,
    columns: Vec<String>
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

    pub fn add_where(self, clause: &str) -> Self {
        let mut s = self;
        s.where_clauses.push(clause.to_string());
        return s;
    }

    pub fn add_where_subquery(self, clause: &str, sb: SelectBuilder) -> Self {
        let mut s: SelectBuilder = self;

        let clause = format!("{} ({})", clause, sb.build());

        s.where_clauses.push(clause);

        return s;
    }

    pub fn and(self) -> Self {
        self.add_where("AND")
    }
    pub fn or(self) -> Self {
        self.add_where("OR")
    }

    pub fn set_limit(self, l1: u32, l2: Option<u32>) -> Self {
        let mut s = self;
        s.limit = Some((l1, l2));
        return s;
    }

    pub fn set_order_by(self, ob1: &str, ob2: Option<&str>) -> Self {
        let mut s = self;
        let ob_dir = match ob2 {
            Some(o) => o,
            None => "ASC"
        };
        s.order_by = Some((ob1.to_string(), ob_dir.to_string()));

        return s;
    }

    pub fn build(&self) -> String {

        let part1 = format!("SELECT {} FROM {}", self.columns.join(", "), self.table);

        let mut wheres: String = String::from("");
        if (self.where_clauses.len() > 0){
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


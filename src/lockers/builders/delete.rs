use super::select::SelectBuilder;


pub struct DeleteBuilder {
    where_clauses: Vec<String>,
    table: String,
}

impl DeleteBuilder {
    pub fn new(table: &str) -> Self {
        Self {
            where_clauses: Vec::new(),
            table: table.to_string(),
        }
    }

    pub fn add_where(self, clause: &str) -> Self {
        let mut s = self;
        s.where_clauses.push(clause.to_string());
        s
    }

    pub fn add_where_subquery(self, clause: &str, sb: SelectBuilder) -> Self {
        let mut s: DeleteBuilder = self;

        let clause = format!("{} ({})", clause, sb.build());

        s.where_clauses.push(clause);
        s
    }

    pub fn and(self) -> Self {
        self.add_where("AND")
    }
    pub fn or(self) -> Self {
        self.add_where("OR")
    }

    pub fn build(&self) -> String {

        let part1 = format!("DELETE FROM {}", self.table);

        let mut wheres: String = String::from("");
        if self.where_clauses.len() > 0{
            wheres = format!(" WHERE {}", self.where_clauses.join(" "))
        }
       

        

        let part2 = format!("{}", wheres);

        part1 + &part2



    }
}


mod lockers;
mod common;

use common::traits::{FromLockerRow, QueryData};
use lockers::mysql_locker::MysqlConnection;
use mysql::from_row;
use std::result::Result;
use std::error::Error;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub fn connect() -> MysqlConnection {
    let mut conn: MysqlConnection = MysqlConnection::new(
        "root".to_string(), 
        "rootroot".to_string(), 
        "localhost".to_string(), 
        3306, 
        "rust_example".to_string()
    );

    return conn;
    

}


pub struct TestDataStruct {
    id: Option<u32>,
    Name: String,
    Address: String,
    Age: u8
}

impl Default for TestDataStruct {
    fn default() -> TestDataStruct {
        TestDataStruct {
            id: None,
            Name: String::from(""),
            Address: String::from(""),
            Age: 0
        }
    }
}

impl ToString for TestDataStruct {
    fn to_string(&self) -> String {
        match self.id {
            Some(id) => format!("{} {} {} {}", id, self.Name, self.Address, self.Age),
            None => format!("{} {} {}", self.Name, self.Address, self.Age)
        }

    }
}

impl QueryData for TestDataStruct {
    fn to_query_string(&self) -> String {
        format!("'{}', '{}', {}", self.Name, self.Address, self.Age)
    }
    fn to_column_array(&self) -> &[&str] {
        &["Name", "Address", "Age"]
    }
}

impl FromLockerRow for TestDataStruct {
    fn from_row(row: mysql::Row) -> Self {
        let (id, name, address, age) = from_row::<(u32, String, String, u8)>(row);
        Self {
            id: Some(id),
            Name: name,
            Address: address,
            Age: age
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::lockers::builders::select::SelectBuilder;

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        let mut conn = connect();
        let r2: Result<(), Box<dyn Error>> = conn.connect();
        //println!(r2.to_string());
        
        assert_eq!(r2.is_ok(), true);

        let rdrop: Result<(), Box<dyn Error>> = conn.drop_table("testTable");
        assert_eq!(rdrop.is_ok(), true);

        let r3 = conn.create("testTable", 
                                        &[
                                            ("id", "<primary id>"),
                                            ("Name", "varchar(255)"),
                                            ("Address", "varchar(255)"),
                                            ("Age", "tinyint UNSIGNED")
                                        ]);

        assert_eq!(r3.is_ok(), true);
        
        let data = [
            TestDataStruct{ Name: String::from("Michael"), Address: String::from("158 Crystal Avenue"), Age: 25, ..Default::default() },
            TestDataStruct{ Name: String::from("John"), Address: String::from("742 St. Lawrence Avenue"), Age: 28, ..Default::default() },
            TestDataStruct{ Name: String::from("William"), Address: String::from("415 Atkins Street"), Age: 55, ..Default::default() }
        ];

        let r4 = conn.insert("testTable", &data);

        assert_eq!(r4.is_ok(), true);

        let r5 = conn.select_raw("testTable", "*", None, None, None);
        //println!("{}", r5);
        assert_eq!(r5.is_ok(), true);
        let rows: Vec<mysql::Row> = r5.unwrap();

        for row in rows {
            let (id, name, address, age) = from_row::<(u32, String, String, u8)>(row);
            
            println!("{} {} {} {}", id, name, address, age);
        }

        let mut selector = SelectBuilder::new("testTable", &["*".to_string()])
            .add_where("Name = 'Michael'")
            .or()
            .add_where("Age = 28");

        //println!("{}", selector.build());

        let r6 = conn.select(&selector);
        assert_eq!(r6.is_ok(), true);
        let rows2: Vec<mysql::Row> = r6.unwrap();

        for row in rows2 {
            let (id, name, address, age) = from_row::<(u32, String, String, u8)>(row);
            
            println!("{} {} {} {}", id, name, address, age);
        }
        let mut subselcetor = SelectBuilder::new("testTable", &["id".to_string()])
            .add_where("id = 3")
            .or()
            .add_where("id = 2");
        let mut selector2 = SelectBuilder::new("testTable", &["Name, Address".to_string()])
            .add_where_subquery("id in", subselcetor)
            .set_order_by("Name", None)
            .set_limit(1, None);

        let r7 = conn.select(&selector2);
        assert_eq!(r7.is_ok(), true);
        let rows3: Vec<mysql::Row> = r7.unwrap();

        for row in rows3 {
            let (name, address) = from_row::<(String, String)>(row);
            
            println!("{} {}", name, address);
        }

        let r8 = conn.select_raw("testTable", "*", None, None, None);
        //println!("{}", r5);
        assert_eq!(r8.is_ok(), true);
        let rows4: Vec<mysql::Row> = r8.unwrap();

        for row in rows4 {
            //let (id, name, address, age) = from_row::<(u32, String, String, u8)>(row);
            let tds = TestDataStruct::from_row(row);
            println!("{}", tds.to_string());
        }

    }
}

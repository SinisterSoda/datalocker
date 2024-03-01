pub mod lockers;
pub mod common;
pub extern crate mysql;

use common::traits::{FromLockerRow, QueryData};

use mysql::from_row;






struct TestDataStruct {
    id: Option<u32>,
    name: String,
    address: String,
    age: u8
}

impl Default for TestDataStruct {
    fn default() -> TestDataStruct {
        TestDataStruct {
            id: None,
            name: String::from(""),
            address: String::from(""),
            age: 0
        }
    }
}

impl ToString for TestDataStruct {
    fn to_string(&self) -> String {
        match self.id {
            Some(id) => format!("{} {} {} {}", id, self.name, self.address, self.age),
            None => format!("{} {} {}", self.name, self.address, self.age)
        }

    }
}

impl QueryData for TestDataStruct {
    fn to_query_string(&self) -> String {
        format!("'{}', '{}', {}", self.name, self.address, self.age)
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
            name,
            address,
            age
        }
    }
}

#[cfg(test)]
mod tests {

    use std::error::Error;

    use crate::{common::{enums::ClauseType, traits::BuildsClauses}, lockers::builders::clause::{ClauseBuilder, SelectBuilder}};
    use lockers::mysql_locker::MysqlConnection;

    use super::*;

    fn connect() -> MysqlConnection {
        let conn: MysqlConnection = MysqlConnection::new(
            "root".to_string(), 
            "rootroot".to_string(), 
            "localhost".to_string(), 
            3306, 
            "rust_example".to_string()
        );
    
        conn
        
    
    }

    #[test]
    fn it_works() {
        let mut conn = connect();
        let r2: Result<(), Box<dyn Error>> = conn.connect();
        //println!(r2.to_string());
        
        assert_eq!(r2.is_ok(), true);

        let rdrop: Result<(), Box<dyn Error>> = conn.drop_table("testTable");
        assert_eq!(rdrop.is_ok(), true);

        let r3 = conn.create("testTable", 
                                        &[
                                            ("id", query_primary_key!()),
                                            ("Name", "varchar(255)"),
                                            ("Address", "varchar(255)"),
                                            ("Age", "tinyint UNSIGNED")
                                        ]);

        assert_eq!(r3.is_ok(), true);
        
        let data = [
            TestDataStruct{ name: String::from("Michael"), address: String::from("158 Crystal Avenue"), age: 25, ..Default::default() },
            TestDataStruct{ name: String::from("John"), address: String::from("742 St. Lawrence Avenue"), age: 28, ..Default::default() },
            TestDataStruct{ name: String::from("William"), address: String::from("415 Atkins Street"), age: 55, ..Default::default() },
            TestDataStruct{ name: String::from("Connor"), address: String::from("84 Brecker Street"), age: 16, ..Default::default() },
            TestDataStruct{ name: String::from("Franz"), address: String::from("481 High Street"), age: 28, ..Default::default() }
        ];

        let r4 = conn.insert("testTable", &data);

        assert_eq!(r4.is_ok(), true);

        let r5 = conn.select_raw("testTable", "*", Some("1=1"), None, None);
        //println!("{}", r5);
        assert_eq!(r5.is_ok(), true);
        let rows: Vec<mysql::Row> = r5.unwrap();

        for row in rows {
            let (id, name, address, age) = from_row::<(u32, String, String, u8)>(row);
            
            println!("{} {} {} {}", id, name, address, age);
        }

        let selector = SelectBuilder::new("testTable", &["*".to_string()])
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
        let subselcetor = SelectBuilder::new("testTable", &["id".to_string()])
            .add_where("id = 3")
            .or()
            .add_where("id = 2");
        let selector2 = SelectBuilder::new("testTable", &["Name, Address".to_string()])
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

        let deletor = ClauseBuilder::new("testTable", ClauseType::Delete)
            .add_where("id = 1");

        let r9 = conn.delete(&deletor);
        assert_eq!(r9.is_ok(), true);
        let r10 = conn.delete_raw("testTable", Some("id=2"));
        assert_eq!(r10.is_ok(), true);


        let r8 = conn.select_raw("testTable", "*", None, None, None);
        //println!("{}", r5);
        assert_eq!(r8.is_ok(), true);
        let rows4: Vec<mysql::Row> = r8.unwrap();

        for row in rows4 {
            //let (id, name, address, age) = from_row::<(u32, String, String, u8)>(row);
            let tds = TestDataStruct::from_row(row);
            println!("{}", tds.to_string());
        }

        let r11 = conn.update_raw("testTable", Some("id = 3"), &[
            ("Name", "'George'"),
            ("Address", "'815 Atkins Street'")
        ]);
        assert_eq!(r11.is_ok(), true);

        let claus = ClauseBuilder::new("testTable", ClauseType::Update)
            .add_where("Name = 'Connor'")
            .or()
            .add_where("Name = 'Franz'");

        let r12 = conn.update(&claus, &[("age", "11")]);
        assert_eq!(r12.is_ok(), true);

        let r8 = conn.select_raw("testTable", "*", None, None, None);
        assert_eq!(r8.is_ok(), true);
        let rows4: Vec<mysql::Row> = r8.unwrap();

        for row in rows4 {
            let tds = TestDataStruct::from_row(row);
            println!("{}", tds.to_string());
        }

        

    }
}

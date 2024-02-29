Basic Database Interface. Currently works with mySQL only. Uses the mysql crate and provides a wrapper around it to make interacting with a mysql database easier.

```
use std::error::Error;

use datalocker::common::traits::QueryData;
use datalocker::common::traits::FromLockerRow;
use datalocker::lockers::builders::select::SelectBuilder;
use datalocker::lockers::mysql_locker::MysqlConnection;
use datalocker::mysql;
use datalocker::query_primary_key;


//It is useful to create structs representing your data
pub struct TestDataStruct {
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

//You can implement the QueryData trait so that they can be automatically inserted using the insert function
impl QueryData for TestDataStruct {
    fn to_query_string(&self) -> String {
        format!("'{}', '{}', {}", self.name, self.address, self.age)
    }
    fn to_column_array(&self) -> &[&str] {
        &["Name", "Address", "Age"]
    }
    
    fn to_column_string(&self) -> String {
        self.to_column_array().join(", ")
    }
}

//You can implement the FromLockerRow trait to automatically create structs from query results
impl FromLockerRow for TestDataStruct {
    fn from_row(row: mysql::Row) -> Self {
        let (id, name, address, age) = mysql::from_row::<(u32, String, String, u8)>(row);
        Self {
            id: Some(id),
            name,
            address,
            age
        }
    }
}

fn main() {

    //Create a new connection
    let mut conn: MysqlConnection = MysqlConnection::new(
        "root".to_string(), 
        "rootroot".to_string(), 
        "localhost".to_string(), 
        3306, 
        "rust_example".to_string()
    );

    let r2: Result<(), Box<dyn Error>> = conn.connect();
    //println!(r2.to_string());

    //Drop a table
    let rdrop: Result<(), Box<dyn Error>> = conn.drop_table("testTable");

    //Create a table. 
    //query_primary_key is a macro that returns a standard primary key field
    //IE int not null PRIMARY KEY AUTO_INCREMENT
    let r3 = conn.create("testTable", 
        &[
            ("id", query_primary_key!()),
            ("Name", "varchar(255)"),
            ("Address", "varchar(255)"),
            ("Age", "tinyint UNSIGNED")
        ]);


    //This is the data to be inserted
    let data = [
            TestDataStruct{ name: String::from("Michael"), address: String::from("158 Crystal Avenue"), age: 25, ..Default::default() },
            TestDataStruct{ name: String::from("John"), address: String::from("742 St. Lawrence Avenue"), age: 28, ..Default::default() },
            TestDataStruct{ name: String::from("William"), address: String::from("415 Atkins Street"), age: 55, ..Default::default() }
        ];

    //insert can be used on structs with the QueryData trait
    let r4 = conn.insert("testTable", &data);

    //a raw select query function. Simply builds a query from the different parts
    //accepts table name, columns to select, a where clause, an order by and a limit
    let r5 = conn.select_raw("testTable", "*", None, None, None);

    //unwrap the results to get a vector of rows
    let rows: Vec<mysql::Row> = r5.unwrap();

    for row in rows {
        //grab the data fields from the row using the mysql crate from_row function
        let (id, name, address, age) = mysql::from_row::<(u32, String, String, u8)>(row);
        
        println!("{} {} {} {}", id, name, address, age);
    }

    //Use SelectBuilder to create more complex select queries that would be unwieldy using the select_row function
    //uses factory style to build the query
    let selector = SelectBuilder::new("testTable", &["*".to_string()])
        .add_where("Name = 'Michael'")
        .or()
        .add_where("Age = 28");

    //select function simply accepts a SelectBuilder object
    let r6 = conn.select(&selector);
    let rows2: Vec<mysql::Row> = r6.unwrap();

    for row in rows2 {
        let (id, name, address, age) = mysql::from_row::<(u32, String, String, u8)>(row);
        
        println!("{} {} {} {}", id, name, address, age);
    }

    //SelectBuilders can accept other select builders are sub queries
    let subselcetor = SelectBuilder::new("testTable", &["id".to_string()])
        .add_where("id = 3")
        .or()
        .add_where("id = 2");
    let selector2 = SelectBuilder::new("testTable", &["Name, Address".to_string()])
        .add_where_subquery("id in", subselcetor)
        .set_order_by("Name", None)
        .set_limit(1, None);
    //selector2 would create the query:
    //SELECT Name, Address FROM testTable WHERE id in (SELECT id FROM testTable WHERE id = 3 OR id = 2) ORDER BY Name ASC LIMIT 1

    let r7 = conn.select(&selector2);
    let rows3: Vec<mysql::Row> = r7.unwrap();

    for row in rows3 {
        let (name, address) = mysql::from_row::<(String, String)>(row);
        
        println!("{} {}", name, address);
    }

    
    let r8 = conn.select_raw("testTable", "*", None, None, None);
    let rows4: Vec<mysql::Row> = r8.unwrap();

    for row in rows4 {
        //when using a struct that implements the FromLockerRow trait
        //new instances can be created using the from_row static method
        let tds = TestDataStruct::from_row(row);
        println!("{}", tds.to_string());
    }
}
```


use crate::common::enums::{MysqlAuthType, DB};
use crate::common::traits::{BuildsClauses, QueryData};
use crate::lockers::builders::mysql::MySqlBuilder;
use mysql::*;
use mysql::prelude::*;

use super::builders::clause::{ClauseBuilder, SelectBuilder};
use super::query_builder::QueryBuilder;





pub struct MysqlConnection {
    pub auth_type: MysqlAuthType,
    pub user: String,
    pub password: String,
    pub server: String,
    pub port: u16,
    pub db_name: String,
    connection: Option<PooledConn>,
    pool: Option<Pool>,
    pub builder:QueryBuilder<MySqlBuilder>


}

impl Default for MysqlConnection {
    fn default() -> Self {
        Self {
            port: 3306,
            server: "localhost".to_string(),
            user: "root".to_string(),
            auth_type: MysqlAuthType::default(),
            password: "root".to_string(),
            db_name: "database".to_string(),
            connection: None,
            pool: None,
            builder: QueryBuilder::<MySqlBuilder>::new(DB::MySQL)

        }
    }
}

impl MysqlConnection {

    pub fn connect(&mut self ) -> std::result::Result<(), Box<dyn std::error::Error>> {

        let url: String = format!("mysql://{}:{}@{}:{}/{}", self.user, self.password, self.server, self.port, self.db_name);
        self.pool = Some(Pool::new(url.as_str())?);
        let temp_pool = &mut self.pool;
        self.connection = match temp_pool{
            None => None,
            Some(p) => Some(p.get_conn()?)
        };

        Ok(())
    }

    pub fn new(user: String, password: String, server: String, port: u16, db_name: String) -> Self {
        Self {
            user,
            password,
            server,
            port,
            db_name,
            auth_type: MysqlAuthType::Sha2,
            pool: None,
            connection: None,
            builder: QueryBuilder::<MySqlBuilder>::new(DB::MySQL)
        }
    }

    pub fn query(&mut self, q: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let temp_conn = &mut self.connection;
        match temp_conn {
            None => {
                eprintln!("No Connection exists");
            }
            Some(conn) => {
                conn.query_drop(q)?;
            }
        }


        Ok(())
    }

    pub fn exec<T: FromRow>(&mut self, q: &str) -> std::result::Result<Vec<T>, Box<dyn std::error::Error>>{
        let temp_conn = &mut self.connection;
        match temp_conn {
            None => {
                Err("Could not complete query".into())
            }
            Some(conn) => {
                let rows: Vec<T> = conn.query(q)?;
                Ok(rows)
            }
        }

        

        
    }

    pub fn drop_table(&mut self, table: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let q = format!("DROP TABLE IF EXISTS {}", table);

        self.query(&q)
    }

    pub fn create(&mut self, table: &str, fields: &[(&str, &str)]) -> std::result::Result<(), Box<dyn std::error::Error>> {

        let q = self.builder.create(table, fields);

        return self.query(&q)

        
    }

    pub fn insert<Q: QueryData>(&mut self, table: &str,  data: &[Q]) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let q = self.builder.insert(table, data);


        return self.query(&q);

    }

    pub fn select_raw(&mut self, table: &str, cols: &str, where_clause: Option<&str>,  order_by: Option<&str>, limit: Option<&str>) -> std::result::Result<Vec<mysql::Row>, Box<dyn std::error::Error>> {
        let q = self.builder.select_raw(table, cols, where_clause, order_by, limit);

        let rows: std::result::Result<Vec<mysql::Row>, Box<dyn std::error::Error>>= self.exec(&q);
        Ok(rows.unwrap())
    }

    pub fn select(&mut self, select_obj: &SelectBuilder) -> std::result::Result<Vec<mysql::Row>, Box<dyn std::error::Error>> {
        let q = self.builder.select(select_obj);
        let rows: std::result::Result<Vec<mysql::Row>, Box<dyn std::error::Error>>= self.exec(&q);
        Ok(rows.unwrap())
    }

    pub fn delete_raw(&mut self, table: &str, where_clause: Option<&str>) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let q = self.builder.delete_raw(table, where_clause);

        self.query(&q)
    }

    pub fn delete(&mut self, select_obj: &ClauseBuilder) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let q = self.builder.delete(select_obj);
        
        self.query(&q)
    }

    pub fn update_raw(&mut self, table: &str, where_clause: Option<&str>, fields: &[(&str, &str)]) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let q = self.builder.update_raw(table, where_clause, fields);
        println!("{}", q);


        self.query(&q)
    }
    
    pub fn update<U: BuildsClauses>(&mut self, clause_obj: &U, field_updates: &[(&str, &str)]) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let q = self.builder.update(clause_obj, field_updates);
        println!("{}", q);


        self.query(&q)
    }

    


}


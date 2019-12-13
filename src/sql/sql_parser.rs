use std::fs::File;
use std::io::Read;
use nom_sql::{parser, SqlQuery};
use std::path::Path;
use std::ops::FnOnce;
use std::borrow::Borrow;
use std::vec::Vec;
use crate::sql::file_to_sql::read_sql_file;


pub fn parse_sql_file(path: &str) -> Vec<SqlQuery> {
    let sql_list = read_sql_file(path);
    let mut results: Vec<SqlQuery> = Vec::new();
    for sql in sql_list {
        let result = parser::parse_query(sql);
        match result {
            Ok(query) => {
                println!("{:?}", &query);
                results.push(query);
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
    return results;
}



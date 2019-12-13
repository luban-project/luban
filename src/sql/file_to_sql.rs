
use std::fs::File;
use std::path::Path;
use std::io::Read;

fn valid_sql_line(line: &&str) -> bool {
    let line = line.trim();
    return !line.is_empty()
        && !line.starts_with("#")
        && !line.starts_with("--")
        && !line.starts_with("//")
        && !(line.starts_with("/*") && line.ends_with("*/;"));
}

pub fn read_sql_file(path: &str) -> Vec<String> {
    let mut sql_file = File::open(Path::new(path))
        .expect("file not found");

    let mut sql_lines = String::new();
    sql_file.read_to_string(&mut sql_lines)
        .expect("sql file read error");
    return split_into_sql(&sql_lines);
}

pub fn split_into_sql(content: &str) -> Vec<String> {
    let lines: Vec<&str> = content
        .lines()
        .map(str::trim)
        .filter( valid_sql_line)
        .collect();
    let mut queries = Vec::new();
    let mut query = String::new();
    for l in lines {
        if l.ends_with(";") {
            // end of query
            query.push_str(l);
            queries.push(query.clone());
            query = String::new();
        } else {
            query.push_str(l);
            query.push_str(" ");
        }
    }
    return queries;
}

#[test]
fn test_split_into_sql() {
    let content = " select * from \n hello; create table hello;";
    let sql_list = split_into_sql(content);
    assert_eq!(sql_list, vec!["select * from hello;", "create table hello;"])
}

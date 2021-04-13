use mysql::prelude::*;
use mysql::*;

#[derive(Debug, PartialEq, Eq)]
pub struct EDRSystemRecord {
    fieldname: String,
    value: String,
}

pub fn get_edrsystem_records() -> Result<Vec<EDRSystemRecord>> {
    let url = "mysql://root@localhost/edr";

    let pool = Pool::new(url);

    let conn = pool.unwrap().get_conn();

    return conn.unwrap().query_map(
        "SELECT fieldname, value from edrsystem",
        |(fieldname, value)| EDRSystemRecord { fieldname, value },
    );
}

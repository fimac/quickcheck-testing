// use postgres::{Client, NoTls};
use quickcheck::Arbitrary;
use quickcheck::Gen;
use quickcheck_macros::quickcheck;
use rand::seq::SliceRandom;
use rand::Rng;

const VALID_COLUMN_NAMES: &[&str] = &["name", "age", "address"];
// const VALID_OPERATORS: &[&str] = &["=", ">", "<", ">=", "<="];
const VALID_OPERATORS: &[&str] = &["="];

#[derive(Debug, Clone)]
struct SqlColumn {
    name: String,
    value: String,
}

#[derive(Debug, Clone)]
struct SqlWhereClause {
    column: SqlColumn,
    operator: String,
}

#[derive(Debug, Clone)]
struct SqlSelectStatement {
    columns: Vec<SqlColumn>,
    where_clauses: Vec<SqlWhereClause>,
}

impl Arbitrary for SqlColumn {
    fn arbitrary(g: &mut Gen) -> SqlColumn {
        // let mut rng = rand::thread_rng();
        // let mut x = rand::thread_rng();
        // let name = VALID_COLUMN_NAMES.choose(&mut rng).unwrap().to_string();
        // let value = match name.as_str() {
        //     "name" => Arbitrary::arbitrary(g),
        //     "age" => format!("{}", x.gen_range(0..90)),
        //     "address" => "1428 Elm St, Springwood".to_string(),
        //     _ => "".to_string(),
        // };
        SqlColumn {
            name: "".to_string(),
            value: "".to_string(),
        }
    }
}

impl Arbitrary for SqlWhereClause {
    fn arbitrary(g: &mut Gen) -> SqlWhereClause {
        let mut rng = rand::thread_rng();
        let column = Arbitrary::arbitrary(g);
        let operator = VALID_OPERATORS.choose(&mut rng).unwrap().to_string();
        SqlWhereClause { column, operator }
    }
}
impl Arbitrary for SqlSelectStatement {
    fn arbitrary(g: &mut Gen) -> SqlSelectStatement {
        let mut rng = rand::thread_rng();
        let num_columns = rng.gen_range(1..=VALID_COLUMN_NAMES.len());
        let mut shuffled_columns: Vec<_> = VALID_COLUMN_NAMES.iter().cloned().collect();
        shuffled_columns.shuffle(&mut rng);

        let num_where_clauses = rng.gen_range(1..=VALID_COLUMN_NAMES.len());
        shuffled_columns.shuffle(&mut rng);
        let selected_where_columns = shuffled_columns.iter().take(num_where_clauses);

        SqlSelectStatement {
            columns: shuffled_columns
                .iter()
                .take(num_columns)
                .map(|name| SqlColumn {
                    name: name.to_string(),
                    value: match *name {
                        "name" => Arbitrary::arbitrary(g),
                        "age" => format!("{}", rng.gen_range(0..90)),
                        "address" => "1428 Elm St, Springwood".to_string(),
                        _ => "".to_string(),
                    },
                })
                .collect(),
            where_clauses: selected_where_columns
                .map(|name| {
                    let operator = VALID_OPERATORS.choose(&mut rng).unwrap().to_string();
                    let value = match *name {
                        "name" => "Fiona".to_string(),
                        "age" => format!("{}", rng.gen_range(0..90)),
                        "address" => "1428 Elm St, Springwood".to_string(),
                        _ => "".to_string(),
                    };
                    SqlWhereClause {
                        column: SqlColumn {
                            name: name.to_string(),
                            value,
                        },
                        operator,
                    }
                })
                .collect(),
        }
    }
}

impl SqlSelectStatement {
    fn to_sql(&self) -> String {
        let columns: Vec<String> = self.columns.iter().map(|c| c.name.clone()).collect();
        let wheres: Vec<String> = self
            .where_clauses
            .iter()
            .map(|wc| format!("{} {} '{}'", wc.column.name, wc.operator, wc.column.value))
            .collect();
        format!(
            "SELECT {} FROM users WHERE {}",
            columns.join(", "),
            wheres.join(" AND ")
        )
    }
}

// fn assert_sql_mapping(original: &SqlSelectStatement, transformed: &str) {
//     for column in &original.columns {
//         assert!(transformed.contains(&format!("{}_encrypted", column.name)));
//     }
// }

#[quickcheck]
fn test_sql_mapper(stmt: SqlSelectStatement) -> bool {
    let original_sql = stmt.to_sql();
    println!("{:?}", original_sql);

    true
}

fn main() {
    println!("just doing some testing...");
}

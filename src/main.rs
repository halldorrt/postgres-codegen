use codegen::{postgres::PostgresSchema, schema::DbSchema};

#[tokio::main]
async fn main() {
    let schema = PostgresSchema::new().await;

    let tables = schema.get_tables().await;
    println!("We print the tables:\n");
    println!("{:?}", tables);

    let columns = schema.get_columns().await;
    println!("We print the columns:\n");
    println!("{:?}", columns);

    let enums = schema.get_enums().await;
    println!("We print the enums:\n");
    println!("{:?}", enums);
}

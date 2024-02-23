use std::collections::HashMap;

use async_trait::async_trait;
use tokio_postgres::{Client, Config, NoTls};

use crate::schema::{ColumnDefinition, DbSchema, EnumDefinition, TableDefinition};

pub struct PostgresSchema {
    client: Client,
}

impl PostgresSchema {
    pub async fn new() -> PostgresSchema {
        let (client, connection) = Config::new()
            .host("localhost")
            .port(5432)
            .user("postgres")
            .password("leviosa")
            .dbname("leviosa")
            .connect(NoTls)
            .await
            .unwrap();

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        PostgresSchema { client }
    }

    async fn columns(&self) -> HashMap<u32, Vec<ColumnDefinition>> {
        let rows = self
            .client
            .query(
                r#"
                    select 
                      tbl.oid as table_oid, 
                      c.column_name,
                      c.data_type, 
                      c.is_nullable
                    from information_schema.columns c
                    left join pg_namespace nsp
                      on nspname = c.table_schema
                    left join pg_class tbl
                      on tbl.relname = c.table_name
                      and tbl.relnamespace = nsp.oid
                    where c.table_schema not in ('information_schema', 'pg_catalog')
                      and c.table_schema not like 'pg_toast%'
                    order by table_oid;
                "#,
                &[],
            )
            .await
            .unwrap();

        rows.into_iter().fold(HashMap::new(), |mut map, row| {
            let table_oid: u32 = row.get(0);
            let name: String = row.get(1);
            let data_type: String = row.get(2);
            let is_nullable: &str = row.get(3);

            map.entry(table_oid).or_default().push(ColumnDefinition {
                table_oid,
                name,
                data_type,
                is_nullable: match { is_nullable } {
                    "YES" => true,
                    "NO" => false,
                    _ => panic!("Invalid value for is_nullable"),
                },
            });
            map
        })
    }
}

#[async_trait]
impl DbSchema for PostgresSchema {
    async fn get_enums(&self) -> Vec<EnumDefinition> {
        let rows = self
            .client
            .query(
                r#"
                    select
                      t.oid,
                      n.nspname as schema_name,
                      t.typname as name,
                      array_agg(e.enumlabel) as values
                    from pg_type t
                    join pg_enum e
                      on e.enumtypid = t.oid
                    join pg_namespace n
                      on n.oid = t.typnamespace
                    group by t.oid, schema_name, name
                    order by schema_name, name;
                "#,
                &[],
            )
            .await
            .unwrap();

        rows.into_iter()
            .map(|row| {
                let oid: u32 = row.get(0);
                let schema: String = row.get(1);
                let name: String = row.get(2);
                let values: Vec<String> = row.get(3);

                EnumDefinition {
                    oid,
                    schema,
                    name,
                    values,
                }
            })
            .collect()
    }

    async fn get_tables(&self) -> Vec<TableDefinition> {
        let rows = self
            .client
            .query(
                r#"
                    select 
                      tbl.oid, 
                      nsp.nspname as schema_name,
                      tbl.relname as table_name
                    from pg_class tbl
                    join pg_namespace nsp 
                      on nsp.oid = tbl.relnamespace
                    where nsp.nspname not in ('information_schema', 'pg_catalog')
                      and nsp.nspname not like 'pg_toast%'
                      and tbl.relkind = 'r';
                "#,
                &[],
            )
            .await
            .unwrap();
        let mut columns = self.columns().await;

        rows.into_iter()
            .map(|row| {
                let oid: u32 = row.get(0);
                let schema: String = row.get(1);
                let name: String = row.get(2);

                TableDefinition {
                    oid,
                    schema,
                    name,
                    columns: columns.remove(&oid).unwrap(),
                }
            })
            .collect()
    }
}

// select nsp.nspname as object_schema,
//        cls.relname as object_name,
//        rol.rolname as owner,
//        case cls.relkind
//          when 'r' then 'TABLE'
//          when 'm' then 'MATERIALIZED_VIEW'
//          when 'i' then 'INDEX'
//          when 'S' then 'SEQUENCE'
//          when 'v' then 'VIEW'
//          when 'c' then 'TYPE'
//          else cls.relkind::text
//        end as object_type
// from pg_class cls
//   join pg_roles rol on rol.oid = cls.relowner
//   join pg_namespace nsp on nsp.oid = cls.relnamespace
// where nsp.nspname not in ('information_schema', 'pg_catalog')
//   and nsp.nspname not like 'pg_toast%'
//   and rol.rolname = current_user  --- remove this if you want to see all objects
// order by nsp.nspname, cls.relname;

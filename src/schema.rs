use std::collections::HashMap;

use async_trait::async_trait;

#[async_trait]
pub trait DbSchema {
    async fn get_enums(&self) -> Vec<EnumDefinition>;

    async fn get_tables(&self) -> Vec<TableDefinition>;

    async fn get_columns(&self) -> HashMap<u32, Vec<ColumnDefinition>>;
}

#[derive(Debug)]
pub struct TableDefinition {
    pub oid: u32,
    pub schema: String,
    pub name: String,
}

#[derive(Debug)]
pub struct ColumnDefinition {
    pub table_oid: u32,
    pub name: String,
    pub data_type: String,
    pub is_nullable: bool,
}

#[derive(Debug)]
pub struct EnumDefinition {
    pub oid: u32,
    pub schema: String,
    pub name: String,
    pub values: Vec<String>,
}

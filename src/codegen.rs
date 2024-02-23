use convert_case::{Case, Casing};

use crate::{
    file_writer::{File, Folder},
    schema::EnumDefinition,
};
use std::collections::HashMap;

pub fn enum_codegen(enums: Vec<EnumDefinition>) -> Vec<Folder> {
    let enums_by_schema: HashMap<String, Vec<EnumDefinition>> =
        enums.into_iter().fold(HashMap::new(), |mut acc, e| {
            let schema = e.schema.clone();
            acc.entry(schema).or_default().push(e);
            acc
        });

    let folders = enums_by_schema
        .into_iter()
        .map(|(schema, enums)| Folder {
            path: format!("./src/out/{}/enums/", schema),
            files: {
                let mut files: Vec<File> = enums
                    .iter()
                    .map(|e| File {
                        name: format!("{}.rs", e.name),
                        conent: handle_enum(&e),
                    })
                    .collect();

                files.push(File {
                    name: "mod.rs".to_string(),
                    conent: enums
                        .iter()
                        .map(|e| {
                            format!(
                                "pub mod {};\npub use {}::{};\n",
                                e.name,
                                e.name,
                                e.name.to_case(Case::Pascal)
                            )
                        })
                        .collect(),
                });

                files
            },
        })
        .collect();

    folders
}

pub fn handle_enum(e: &EnumDefinition) -> Vec<String> {
    let mut lines = vec![format!("pub enum {} {{", e.name.to_case(Case::Pascal))];
    for variant in &e.values {
        lines.push(format!("    {},", variant.to_case(Case::Pascal)));
    }
    lines.push("}".to_string());
    lines
}

use std::{
    env, fs,
    path::{Path, PathBuf},
};

use typify::{TypeSpace, TypeSpaceSettings};

fn main() {
    // Apply cargo typify to generate struct files for the schema files.
    // The root schema_dir contains a separate directory for each asset type.
    // Each subdirectory contains a JSON schema file for the asset type.
    let schema_dir = Path::new("rpg-generation-assets");
    let schema_dirs: Vec<PathBuf> = schema_dir
        .read_dir()
        .expect("rpg-generation-assets directory not found")
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_dir())
        .collect::<Vec<_>>();
    let schema_files: Vec<PathBuf> = schema_dirs
        .iter()
        .flat_map(|schema_dir| {
            fs::read_dir(schema_dir)
                .unwrap()
                .map(|entry| entry.unwrap().path())
                .filter(|path| path.is_file())
                .filter(|path| path.extension().unwrap() == "json")
                .collect::<Vec<_>>()
        })
        .collect();
    for schema_file in schema_files {
        // Get the schema file name
        let schema_file_name = schema_file.file_name().unwrap().to_str().unwrap();
        let schema_name = schema_file_name.split('.').next().unwrap();
        // Read the contents of the schema file
        let content = fs::read_to_string(&schema_file).unwrap();
        // Parse the schema file into a RootSchema
        let schema = serde_json::from_str::<schemars::schema::RootSchema>(&content)
            .expect(format!("Failed to parse schema file {}", schema_file_name).as_str());
        // Generate the struct file
        let mut type_space = TypeSpace::new(TypeSpaceSettings::default().with_struct_builder(true));
        type_space.add_root_schema(schema).unwrap();
        let contents =
            prettyplease::unparse(&syn::parse2::<syn::File>(type_space.to_stream()).unwrap());
        let mut out_file = Path::new(&env::var("OUT_DIR").unwrap()).to_path_buf();
        out_file.push(format!("{}.rs", schema_name));
        fs::write(out_file, contents).unwrap();
    }
}

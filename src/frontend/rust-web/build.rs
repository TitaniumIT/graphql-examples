fn main() {
    cynic_codegen::register_schema("example")
        .from_sdl_file("src/schema.graphqls")
        .unwrap()
        .as_default()
        .unwrap();
}
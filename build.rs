use dependencies_sync::tonic_build;

fn main() {
    tonic_build::configure()
        .out_dir("src/protocols")
        .extern_path(".cashmere", "::manage_define::cashmere")
        .build_server(true)
        .build_client(false)
        // .type_attribute("Name", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(
            &["protocols/account_service.proto"],
            &["protocols", "../cashmere_core/protocols"],
        )
        .unwrap();

    define_utils::generate_manage_defines(
        &vec!["manage_defines"],
        "src/ids_codes",
        // None,
        Some("dart_packages/account_module/lib"),
        Some("account_module"),
    );
}

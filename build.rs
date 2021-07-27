fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tonic_build::compile_protos("proto/dbscanserving.proto")?;

    tonic_build::configure()
        // .type_attribute("dbscanserving.Sample", "#[derive(Eq, Copy)]")
        // .field_attribute("dbscanserving.Sample.features", "#[derive(Copy)]")
        .compile(&["proto/dbscanserving.proto"], &["proto"])
        .unwrap();

    Ok(())
}

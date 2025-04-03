use anyhow::Result;

fn main() -> Result<()> {
    compile_css()?;
    compile_protobufs()?;

    Ok(())
}

fn compile_css() -> Result<()> {
    std::process::Command::new("npm")
        .args(["run", "build"])
        .env("NODE_ENV", "prodution")
        .spawn()?
        .wait()?;
    Ok(())
}

fn compile_protobufs() -> Result<()> {
    std::env::set_var("PROTOC", protoc_bin_vendored::protoc_bin_path()?);
    std::env::set_var("PROTOBUF_LOCATION", protoc_bin_vendored::include_path()?);
    let protos = &[
        "AuthData.proto",
        "ClientConfigData.proto",
        "ClientLog.proto",
        "ClientMetrics.proto",
        "ClientTelemetry.proto",
        "Common.proto",
        "CustomerServiceData.proto",
        "Error.proto",
        "GambleData.proto",
        "GameplayConfigData.proto",
        "GetFriendData.proto",
        "LandData.proto",
        "MatchmakingData.proto",
        "OffersData.proto",
        "PurchaseData.proto",
        "WholeLandTokenData.proto",
    ];
    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute(".", r#"#[serde(rename_all = "camelCase")]"#)
        .include_file("include_all.rs")
        .compile_protos(protos, &["src/protos"])?;
    Ok(())
}

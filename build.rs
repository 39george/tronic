use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let root_dir = std::env::current_dir().unwrap();

    if env::var_os("CARGO_PRIMARY_PACKAGE").is_none() {
        println!(
            "cargo:warning=tronic: skipping proto codegen (building as dependency)"
        );
        return Ok(());
    }

    if !env::var_os("TRONIC_REGEN_PROTO").is_some() {
        println!(
            "cargo:warning=tronic: build-proto is disabled; using pre-generated files"
        );
        return Ok(());
    }

    tonic_prost_build::configure()
        .out_dir(root_dir.join("src/protocol"))
        .build_server(false)
        .file_descriptor_set_path(out_dir.join("tron_protocol_descriptor.bin"))
        .type_attribute("SmartContract.ABI", "#[cfg_attr(feature = \"serde\", derive(serde::Serialize, serde::Deserialize))]")
        .type_attribute("SmartContract.ABI.Entry", "#[cfg_attr(feature = \"serde\", derive(serde::Serialize, serde::Deserialize))]")
        .type_attribute("SmartContract.ABI.Entry.Param", "#[cfg_attr(feature = \"serde\", derive(serde::Serialize, serde::Deserialize))]")
        .field_attribute("SmartContract.ABI.Entry.anonymous", "#[cfg_attr(feature = \"serde\", serde(default))]")
        .field_attribute("SmartContract.ABI.Entry.constant", "#[cfg_attr(feature = \"serde\", serde(default))]")
        .field_attribute("SmartContract.ABI.Entry.payable", "#[cfg_attr(feature = \"serde\", serde(default))]")
        .field_attribute("SmartContract.ABI.Entry.name", "#[cfg_attr(feature = \"serde\", serde(default))]")
        .field_attribute("SmartContract.ABI.Entry.outputs", "#[cfg_attr(feature = \"serde\", serde(default))]")
        .field_attribute("SmartContract.ABI.Entry.Param.indexed", "#[cfg_attr(feature = \"serde\", serde(default))]")
        .field_attribute("SmartContract.ABI.Entry.stateMutability", "#[cfg_attr(feature = \"serde\", serde(default))]")
        .type_attribute("AccountResourceMessage", "#[cfg_attr(feature = \"serde\", derive(serde::Serialize, serde::Deserialize))]")
        .enum_attribute("Transaction.Result.contractResult", "#[cfg_attr(feature = \"serde\", derive(serde::Serialize, serde::Deserialize))]")
        .enum_attribute("Transaction.Contract.ContractType", "#[cfg_attr(feature = \"serde\", derive(serde::Serialize, serde::Deserialize))]")
        .compile_protos(
            &[
                "proto/tron/api/api.proto",
            ],
            &["proto/tron", "proto"],
        )
        .unwrap();

    // Delete unused file
    std::fs::remove_file(root_dir.join("src/protocol/google.api.rs"))?;

    Ok(())
}

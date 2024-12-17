use anvil_zksync_config::types::Genesis;
use anvil_zksync_config::TestNodeConfig;
use anvil_zksync_core::fork::ForkDetails;
use anyhow::Context;
use serde::Serialize;
use std::fs;
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

/// Parses the genesis file from the given path.
pub fn parse_genesis_file(path: &str) -> Result<Genesis, String> {
    let file_content =
        fs::read_to_string(path).map_err(|err| format!("Failed to read file: {err}"))?;
    serde_json::from_str(&file_content).map_err(|err| format!("Failed to parse JSON: {err}"))
}

/// Writes the given serializable object as JSON to the specified file path using pretty printing.
/// Returns an error if the file cannot be created or if serialization/writing fails.
pub fn write_json_file<T: Serialize>(path: &Path, obj: &T) -> anyhow::Result<()> {
    let file = File::create(path)
        .with_context(|| format!("Failed to create file '{}'", path.display()))?;
    let mut writer = BufWriter::new(file);
    // Note: intentionally using pretty printing for better readability.
    serde_json::to_writer_pretty(&mut writer, obj)
        .with_context(|| format!("Failed to write JSON to '{}'", path.display()))?;
    writer
        .flush()
        .with_context(|| format!("Failed to flush writer for '{}'", path.display()))?;

    Ok(())
}

/// Updates the configuration from fork details.
pub async fn update_with_fork_details(
    config: &mut TestNodeConfig,
    fork_details_result: Result<ForkDetails, eyre::Report>,
) -> Result<Option<ForkDetails>, anyhow::Error> {
    match fork_details_result {
        Ok(fd) => {
            let l1_gas_price = config.l1_gas_price.or(Some(fd.l1_gas_price));
            let l2_gas_price = config.l2_gas_price.or(Some(fd.l2_fair_gas_price));
            let l1_pubdata_price = config.l1_pubdata_price.or(Some(fd.fair_pubdata_price));
            let price_scale = config
                .price_scale_factor
                .or(Some(fd.estimate_gas_price_scale_factor));
            let gas_limit_scale = config
                .limit_scale_factor
                .or(Some(fd.estimate_gas_scale_factor));
            let chain_id = config.chain_id.or(Some(fd.chain_id.as_u64() as u32));

            config
                .update_l1_gas_price(l1_gas_price)
                .update_l2_gas_price(l2_gas_price)
                .update_l1_pubdata_price(l1_pubdata_price)
                .update_price_scale(price_scale)
                .update_gas_limit_scale(gas_limit_scale)
                .update_chain_id(chain_id);

            Ok(Some(fd))
        }
        Err(error) => {
            tracing::error!("Error while attempting to fork: {:?}", error);
            Err(anyhow::anyhow!(error))
        }
    }
}

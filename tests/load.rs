use anyhow::{ensure, Result};
use serde::Deserialize;
use std::{fs, path::PathBuf};

#[derive(Deserialize)]
struct Env {
    pub cargo_manifest_dir: PathBuf,
}

#[derive(Deserialize)]
struct Config {
    pub dataset_dir: PathBuf,
}

#[test]
fn parsing_test() -> Result<()> {
    // load config
    let Config { dataset_dir } = {
        let Env { cargo_manifest_dir } = envy::from_env()?;
        let config_file = cargo_manifest_dir.join("tests").join("test_config.json5");

        ensure!(
            config_file.is_file(),
            "please create an config file for testing at '{}'. an example file can be found in the same directory",
            config_file.display()
        );

        json5::from_str(&fs::read_to_string(&config_file)?)?
    };

    // load dataset
    let _samples = iii_formosa_dataset::load(&dataset_dir)?;

    Ok(())
}

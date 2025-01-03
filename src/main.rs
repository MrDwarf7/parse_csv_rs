mod cli;
mod config;
mod error;
mod prelude;
mod processor;
mod retained;

use crate::cli::{Cli, OutputType};
use crate::config::Config;
pub(crate) use crate::prelude::*;
use crate::processor::Processor;
use crate::retained::RetainedData;

#[derive(Debug)]
struct OutputData {
    output_type: OutputType,
    output_path: PathBuf,
}
impl OutputData {
    fn new(output_type: OutputType, output_path: PathBuf) -> Self {
        Self {
            output_type,
            output_path,
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::new();
    let config = Config::new(&cli)?;

    // Store output type for later
    let output_data = OutputData::new(config.output_type, config.output_path.clone());
    // let output_type = cli.output_type.unwrap_or(config.output_type);

    let mut retained_data = RetainedData::default();

    let processor = Processor::new(config, &mut retained_data)?;
    if let Err(e) = processor.process(&mut retained_data) {
        eprintln!("Error processing: {}", e);
    }

    match output_data.output_type {
        OutputType::Stdout => {
            retained_data.to_stdout()?;
        }
        OutputType::Csv => {
            retained_data.to_csv(output_data.output_path)?;
        }
    }

    Ok(())
}

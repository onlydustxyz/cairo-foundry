use std::{fmt::Display, path::PathBuf};

use clap::{Args, ValueHint};
use cleopatra_cairo::cairo_run;
use serde::Serialize;

use super::CommandExecution;

#[derive(Args, Debug)]
pub struct ExecuteArgs {
	/// Path to a json compiled cairo program
	#[clap(short, long, value_hint=ValueHint::FilePath, value_parser=is_json)]
	program: PathBuf,
}

fn is_json(path: &str) -> Result<PathBuf, String> {
	let path = PathBuf::from(path);
	if path.exists() && path.is_file() {
		match path.extension() {
			Some(ext) if ext == "json" => Ok(path),
			_ => Err(format!("\"{}\" is not a json file", path.display())),
		}
	} else {
		Err(format!("\"{}\" is not a valid file", path.display()))
	}
}

/// Execute command output
#[derive(Debug, Serialize)]
pub struct ExecuteOutput {}

impl Display for ExecuteOutput {
	fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		Ok(())
	}
}

impl CommandExecution<ExecuteOutput> for ExecuteArgs {
	fn exec(&self) -> Result<ExecuteOutput, String> {
		let mut cairo_runner = cairo_run::cairo_run(&self.program).map_err(|e| {
			format!(
				"failed to run the program \"{}\": {}",
				self.program.display(),
				e,
			)
		})?;

		cairo_run::write_output(&mut cairo_runner).map_err(|e| {
			format!(
				"failed to print the program output \"{}\": {}",
				self.program.display(),
				e,
			)
		})?;

		Ok(ExecuteOutput {})
	}
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn valid_programs() {
		assert!(ExecuteArgs {
			program: PathBuf::from(
				"./test_starknet_projects/compiled_programs/valid_program_a.json"
			),
		}
		.exec()
		.is_ok());

		assert!(ExecuteArgs {
			program: PathBuf::from(
				"./test_starknet_projects/compiled_programs/valid_program_b.json"
			),
		}
		.exec()
		.is_ok());
	}

	#[test]
	fn invalid_programs() {
		assert!(ExecuteArgs {
			program: PathBuf::from(
				"./test_starknet_projects/compiled_programs/invalid_odd_length_hex.json"
			),
		}
		.exec()
		.is_err());

		assert!(ExecuteArgs {
			program: PathBuf::from(
				"./test_starknet_projects/compiled_programs/invalid_even_length_hex.json"
			),
		}
		.exec()
		.is_err());
	}
}

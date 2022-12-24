// Copyright 2019-2022 PureStake Inc.
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.

//! This module constructs and executes the appropriate service components for the given subcommand

use crate::cli::{Cli, Subcommand};
use cli_opt::{EthApi, RpcConfig};
use frame_benchmarking_cli::BenchmarkCmd;
//#[cfg(feature = "westend-native")]
use sc_cli::{
	Result, RuntimeVersion, SubstrateCli,
};
use sc_service::DatabaseSource;
use service::{chain_spec, frontier_database_dir, IdentifyVariant};

fn load_spec(
	id: &str,
) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
	Ok(match id {
		// Moonbeam networks
		"moonbeam" | "" => Box::new(chain_spec::RawChainSpec::from_json_bytes(
			&include_bytes!("../../../specs/moonbeam/parachain-embedded-specs.json")[..],
		)?),

		#[cfg(feature = "moonbeam-native")]
		"dev" => Box::new(chain_spec::moonbeam::development_chain_spec(None, None)),

		#[cfg(feature = "moonbeam-native")]
		"local" => Box::new(chain_spec::moonbeam::get_chain_spec()),
		path => {
			let path = std::path::PathBuf::from(path);
            Box::new(chain_spec::moonbeam::ChainSpec::from_json_file(path)?)
		}
	})
}

impl SubstrateCli for Cli {
	fn impl_name() -> String {
		"SPF Mainnet Node".into()
	}

	fn impl_version() -> String {
		env!("SUBSTRATE_CLI_IMPL_VERSION").into()
	}

	fn description() -> String {
		format!(
			"Moonbase Parachain Collator\n\nThe command-line arguments provided first will be \
		passed to the parachain node, while the arguments provided after -- will be passed \
		to the relaychain node.\n\n\
		{} [parachain-args] -- [relaychain-args]",
			Self::executable_name()
		)
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"https://github.com/PureStake/moonbeam/issues/new".into()
	}

	fn copyright_start_year() -> i32 {
		2020
	}

	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
		load_spec(id)
	}

	fn native_runtime_version(spec: &Box<dyn sc_service::ChainSpec>) -> &'static RuntimeVersion {
		match spec {
			#[cfg(feature = "moonbeam-native")]
			spec if spec.is_moonbeam() => return &service::moonbeam_runtime::VERSION,

			_ => panic!("invalid chain spec"),
		}
	}
}


fn validate_trace_environment(cli: &Cli) -> Result<()> {
	if (cli.run.ethapi.contains(&EthApi::Debug) || cli.run.ethapi.contains(&EthApi::Trace))
		&& cli
			.run
			.base
			.import_params
			.wasm_runtime_overrides
			.is_none()
	{
		return Err(
			"`debug` or `trace` namespaces requires `--wasm-runtime-overrides /path/to/overrides`."
				.into(),
		);
	}
	Ok(())
}

/// Parse command line arguments into service configuration.
pub fn run() -> Result<()> {
	let mut cli = Cli::from_args();
	let _ = validate_trace_environment(&cli)?;
	// Set --execution wasm as default
	let execution_strategies = cli.run.base.import_params.execution_strategies.clone();
	if execution_strategies.execution.is_none() {
		cli.run
			.base
			.import_params
			.execution_strategies
			.execution = Some(sc_cli::ExecutionStrategy::Wasm);
	}

	match &cli.subcommand {
		Some(Subcommand::BuildSpec(params)) => {
			let runner = cli.create_runner(&params.base)?;
			runner.sync_run(|config| {
				if params.mnemonic.is_some() || params.accounts.is_some() {
                    params.base.run(
                        Box::new(chain_spec::moonbeam::development_chain_spec(
                            params.mnemonic.clone(),
                            params.accounts,
                        )),
                        config.network,
                    )
				} else {
					params.base.run(config.chain_spec, config.network)
				}
			})
		}
		Some(Subcommand::CheckBlock(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|mut config| {
				let (client, _, import_queue, task_manager) = service::new_chain_ops(&mut config)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		}
		Some(Subcommand::ExportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|mut config| {
				let (client, _, _, task_manager) = service::new_chain_ops(&mut config)?;
				Ok((cmd.run(client, config.database), task_manager))
			})
		}
		Some(Subcommand::ExportState(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|mut config| {
				let (client, _, _, task_manager) = service::new_chain_ops(&mut config)?;
				Ok((cmd.run(client, config.chain_spec), task_manager))
			})
		}
		Some(Subcommand::ImportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|mut config| {
				let (client, _, import_queue, task_manager) = service::new_chain_ops(&mut config)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		}
		Some(Subcommand::PurgeChain(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| {
				let dev_service = cli.run.dev_service;

				// Remove Frontier offchain db
				let frontier_database_config = match config.database {
					DatabaseSource::RocksDb { .. } => DatabaseSource::RocksDb {
						path: frontier_database_dir(&config, "db"),
						cache_size: 0,
					},
					DatabaseSource::ParityDb { .. } => DatabaseSource::ParityDb {
						path: frontier_database_dir(&config, "paritydb"),
					},
					_ => {
						return Err(format!("Cannot purge `{:?}` database", config.database).into())
					}
				};
				cmd.run(frontier_database_config)?;

				if dev_service {
					// base refers to the encapsulated "regular" sc_cli::PurgeChain command
					return cmd.run(config.database);
				}

                Ok(())
			})
		}
		Some(Subcommand::Revert(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;
			match chain_spec {
				#[cfg(feature = "moonbeam-native")]
				spec if spec.is_moonbeam() => runner.async_run(|mut config| {
					let params = service::new_partial::<
						service::moonbeam_runtime::RuntimeApi,
						service::MoonbeamExecutor,
					>(&mut config, false)?;

					Ok((
						cmd.run(params.client, params.backend, None),
						params.task_manager,
					))
				}),
				_ => panic!("invalid chain spec"),
			}
		}
		Some(Subcommand::Benchmark(cmd)) => {
			let runner = cli.create_runner(cmd)?;

			// Switch on the concrete benchmark sub-command
			match cmd {
				BenchmarkCmd::Pallet(cmd) => {
					if cfg!(feature = "runtime-benchmarks") {
						let chain_spec = &runner.config().chain_spec;
						match chain_spec {
							#[cfg(feature = "moonbeam-native")]
							spec if spec.is_moonbeam() => {
								return runner.sync_run(|config| {
									cmd.run::<service::moonbeam_runtime::Block, service::MoonbeamExecutor>(
										config,
									)
								})
							}
							_ => panic!("invalid chain spec"),
						}
					} else {
						Err("Benchmarking wasn't enabled when building the node. \
					You can enable it with `--features runtime-benchmarks`."
							.into())
					}
				}
				BenchmarkCmd::Block(cmd) => {
					let chain_spec = &runner.config().chain_spec;
					match chain_spec {
						#[cfg(feature = "moonbeam-native")]
						spec if spec.is_moonbeam() => {
							return runner.sync_run(|mut config| {
								let params = service::new_partial::<
									service::moonbeam_runtime::RuntimeApi,
									service::MoonbeamExecutor,
								>(&mut config, false)?;

								cmd.run(params.client)
							})
						}
						_ => panic!("invalid chain spec"),
					}
				}
				BenchmarkCmd::Storage(cmd) => {
					let chain_spec = &runner.config().chain_spec;
					match chain_spec {
						#[cfg(feature = "moonbeam-native")]
						spec if spec.is_moonbeam() => {
							return runner.sync_run(|mut config| {
								let params = service::new_partial::<
									service::moonbeam_runtime::RuntimeApi,
									service::MoonbeamExecutor,
								>(&mut config, false)?;

								let db = params.backend.expose_db();
								let storage = params.backend.expose_storage();

								cmd.run(config, params.client, db, storage)
							})
						}
						_ => panic!("invalid chain spec"),
					}
				}
				BenchmarkCmd::Overhead(_) => Err("Unsupported benchmarking command".into()),
				BenchmarkCmd::Extrinsic(_) => Err("Unsupported benchmarking command".into()),
				BenchmarkCmd::Machine(cmd) => {
					return runner.sync_run(|config| {
						cmd.run(
							&config,
							frame_benchmarking_cli::SUBSTRATE_REFERENCE_HARDWARE.clone(),
						)
					});
				}
			}
		}
		Some(Subcommand::Key(cmd)) => Ok(cmd.run(&cli)?),
		None => {
			let runner = cli.create_runner(&(*cli.run))?;
			runner.run_node_until_exit(|config| async move {
				//benchmarks hardware to detective the performance of current machine
				let hwbench = if !cli.run.no_hardware_benchmarks {
					config.database.path().map(|database_path| {
						let _ = std::fs::create_dir_all(&database_path);
						sc_sysinfo::gather_hwbench(Some(database_path))
					})
				} else {
					None
				};

				let rpc_config = RpcConfig {
					ethapi: cli.run.ethapi,
					ethapi_max_permits: cli.run.ethapi_max_permits,
					ethapi_trace_max_count: cli.run.ethapi_trace_max_count,
					ethapi_trace_cache_duration: cli.run.ethapi_trace_cache_duration,
					eth_log_block_cache: cli.run.eth_log_block_cache,
					eth_statuses_cache: cli.run.eth_statuses_cache,
					fee_history_limit: cli.run.fee_history_limit,
					max_past_logs: cli.run.max_past_logs,
					relay_chain_rpc_url: None,
					tracing_raw_max_memory_usage: cli.run.tracing_raw_max_memory_usage,
				};


					// When running the dev service, just use Alice's author inherent
					//TODO maybe make the --alice etc flags work here, and consider bringing back
					// the author-id flag. For now, this will work.

					let tmp_seed = "Alice";
					let author_id = Some(chain_spec::get_from_seed::<nimbus_primitives::NimbusId>(
						tmp_seed,
					));

//					log::info!("config.chain_spec, {:?}", &config.chain_spec);
//					log::info!("author_id: {:?}", author_id);

					return match &config.chain_spec {
						#[cfg(feature = "moonbeam-native")]
						spec if spec.is_moonbeam() => service::new_dev::<
							service::moonbeam_runtime::RuntimeApi,
							service::MoonbeamExecutor,
						>(config, author_id, cli.run.sealing, rpc_config, hwbench)
						.map_err(Into::into),
						_ => panic!("invalid chain spec"),
					};
			})
		}
	}
}


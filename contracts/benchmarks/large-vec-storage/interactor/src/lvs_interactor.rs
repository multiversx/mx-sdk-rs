mod lvs_interactor_cli;
mod lvs_interactor_config;

use std::fmt::Write;

use clap::Parser;
use large_vec_storage::large_vec_storage_proxy;
pub use lvs_interactor_config::Config;

use multiversx_sc_snippets::imports::*;

/// The two output contracts defined in `sc-config.toml`.
pub const VARIANTS: &[Variant] = &[
    Variant {
        label: "default",
        code_path: MxscPath::new("../output/large-vec-storage.mxsc.json"),
    },
    Variant {
        label: "preload",
        code_path: MxscPath::new("../output/large-vec-storage-preload.mxsc.json"),
    },
];

/// Endpoints that only read storage, measured in this order.
pub const READ_ENDPOINTS: &[&str] = &["noop", "loadRaw", "decodeDefault", "iterSum", "encodeOnly"];

/// Mutates storage (measured via simulation, so state is not actually changed).
pub const CLAIM_ENDPOINT: &str = "claimDefault";

/// Max gas per transaction allowed by the protocol.
pub const MAX_GAS: u64 = 600_000_000;

/// Number of entries added per `fill` transaction.
pub const FILL_CHUNK: usize = 50;

pub struct Variant {
    pub label: &'static str,
    pub code_path: MxscPath<'static>,
}

/// Gas used by one call, or `None` if the call failed (typically out of gas).
pub type GasMeasurement = Option<u64>;

pub struct BenchResults {
    pub sizes: Vec<usize>,
    /// Indexed as `[variant][size][endpoint]`, endpoints are `READ_ENDPOINTS` followed by `CLAIM_ENDPOINT`.
    pub gas: Vec<Vec<Vec<GasMeasurement>>>,
}

pub async fn large_vec_storage_cli() {
    env_logger::init();

    let cli = lvs_interactor_cli::InteractCli::parse();
    match &cli.command {
        Some(lvs_interactor_cli::InteractCliCommand::Bench(args)) => {
            let mut interact = LargeVecStorageInteractor::new(Config::load_config()).await;
            let results = interact.bench(&args.sizes).await;
            let report = results.to_markdown();
            println!("\n{report}");
            std::fs::write(&args.output, &report).unwrap();
            println!("Report written to {}", args.output);
        }
        None => {}
    }
}

pub struct LargeVecStorageInteractor {
    pub interactor: Interactor,
    pub owner_address: Bech32Address,
}

impl LargeVecStorageInteractor {
    pub async fn new(config: Config) -> Self {
        let mut interactor = Interactor::new(config.gateway_uri())
            .await
            .use_chain_simulator(config.use_chain_simulator());
        interactor
            .set_current_dir_from_workspace("contracts/benchmarks/large-vec-storage/interactor");

        let owner_address = interactor.register_wallet(test_wallets::mike()).await;

        interactor.generate_blocks(30u64).await.unwrap();

        LargeVecStorageInteractor {
            interactor,
            owner_address: owner_address.into(),
        }
    }

    pub async fn deploy(&mut self, code_path: MxscPath<'_>) -> Bech32Address {
        self.interactor
            .tx()
            .from(&self.owner_address)
            .gas(50_000_000u64)
            .typed(large_vec_storage_proxy::LargeVecStorageBenchmarkProxy)
            .init()
            .code(code_path)
            .returns(ReturnsNewBech32Address)
            .run()
            .await
    }

    /// Resets storage, then fills it with `n` entries, in chunks.
    pub async fn set_entries(&mut self, sc: &Bech32Address, n: usize) {
        self.interactor
            .tx()
            .from(&self.owner_address)
            .to(sc)
            .gas(MAX_GAS)
            .typed(large_vec_storage_proxy::LargeVecStorageBenchmarkProxy)
            .clear()
            .run()
            .await;

        let mut start = 0;
        while start < n {
            let count = FILL_CHUNK.min(n - start);
            self.interactor
                .tx()
                .from(&self.owner_address)
                .to(sc)
                .gas(MAX_GAS)
                .typed(large_vec_storage_proxy::LargeVecStorageBenchmarkProxy)
                .fill(start, count)
                .run()
                .await;
            start += count;
        }
    }

    /// Simulates a call to an endpoint with no arguments, via the gateway `transaction/cost` endpoint.
    ///
    /// Returns the gas consumed, or `None` if the simulation failed (typically out of gas).
    ///
    /// Note: we cannot use `ReturnsGasUsed` on executed transactions, because no gas refund is issued
    /// on the chain simulator, so the reported `gasUsed` is always the gas limit.
    /// The simulation does not change state.
    pub async fn measure(&mut self, sc: &Bech32Address, endpoint: &str) -> GasMeasurement {
        let gas_result = self
            .interactor
            .tx()
            .from(&self.owner_address)
            .to(sc)
            .gas(MAX_GAS)
            .raw_call(endpoint)
            .simulate_gas()
            .await;

        match gas_result {
            GasResult::GasUsed(gas) => Some(gas),
            GasResult::OutOfGas => {
                println!("    {endpoint}: out of gas");
                None
            }
        }
    }

    pub async fn bench(&mut self, sizes: &[usize]) -> BenchResults {
        let mut gas = Vec::new();
        for variant in VARIANTS {
            let sc = self.deploy(variant.code_path).await;
            println!("deployed {} at {sc}", variant.label);

            let mut variant_gas = Vec::new();
            for &n in sizes {
                self.set_entries(&sc, n).await;

                let mut row = Vec::new();
                for endpoint in READ_ENDPOINTS {
                    row.push(self.measure(&sc, endpoint).await);
                }
                row.push(self.measure(&sc, CLAIM_ENDPOINT).await);

                println!("  {} n={n}: {}", variant.label, format_row(&row));
                variant_gas.push(row);
            }
            gas.push(variant_gas);
        }

        BenchResults {
            sizes: sizes.to_vec(),
            gas,
        }
    }
}

fn all_endpoints() -> impl Iterator<Item = &'static str> {
    READ_ENDPOINTS
        .iter()
        .copied()
        .chain(core::iter::once(CLAIM_ENDPOINT))
}

fn endpoint_index(endpoint: &str) -> usize {
    all_endpoints().position(|e| e == endpoint).unwrap()
}

/// Formats gas with thousands separators, or "OOG" when the call failed.
pub fn format_gas(gas: GasMeasurement) -> String {
    match gas {
        Some(gas) => {
            let s = gas.to_string();
            let mut out = String::new();
            for (i, c) in s.chars().enumerate() {
                if i > 0 && (s.len() - i) % 3 == 0 {
                    out.push(',');
                }
                out.push(c);
            }
            out
        }
        None => "OOG".to_string(),
    }
}

fn format_row(row: &[GasMeasurement]) -> String {
    all_endpoints()
        .zip(row)
        .map(|(e, g)| format!("{e}={}", format_gas(*g)))
        .collect::<Vec<_>>()
        .join(" ")
}

fn format_ratio(numerator: GasMeasurement, denominator: GasMeasurement) -> String {
    match (numerator, denominator) {
        (Some(n), Some(d)) if d > 0 => format!("{:.2}x", n as f64 / d as f64),
        _ => "-".to_string(),
    }
}

impl BenchResults {
    pub fn get(&self, variant: usize, size_index: usize, endpoint: &str) -> GasMeasurement {
        self.gas[variant][size_index][endpoint_index(endpoint)]
    }

    /// Gas used, minus the `noop` baseline of the same build.
    ///
    /// The baseline differs between builds, since contract size affects the cost of every call.
    pub fn get_net(&self, variant: usize, size_index: usize, endpoint: &str) -> GasMeasurement {
        let gas = self.get(variant, size_index, endpoint)?;
        let baseline = self.get(variant, size_index, "noop")?;
        Some(gas.saturating_sub(baseline))
    }

    pub fn to_markdown(&self) -> String {
        let mut out = String::new();
        writeln!(out, "# Large ManagedVec storage gas report\n").unwrap();
        writeln!(
            out,
            "Gas consumed per transaction, from the gateway `transaction/cost` simulation (gas limit {}). `OOG` = simulation failed, most likely out of gas.\n",
            format_gas(Some(MAX_GAS))
        )
        .unwrap();

        writeln!(
            out,
            "## Framework decoder: default vs `managed-buffer-preload`\n"
        )
        .unwrap();
        writeln!(
            out,
            "Raw gas per transaction. Ratios are computed net of each build's `noop` baseline.\n"
        )
        .unwrap();
        writeln!(
            out,
            "| N | decode (default) | decode (preload) | ratio | claim (default) | claim (preload) | ratio |"
        )
        .unwrap();
        writeln!(out, "|---|---|---|---|---|---|---|").unwrap();
        for (i, n) in self.sizes.iter().enumerate() {
            let d0 = self.get(0, i, "decodeDefault");
            let d1 = self.get(1, i, "decodeDefault");
            let c0 = self.get(0, i, CLAIM_ENDPOINT);
            let c1 = self.get(1, i, CLAIM_ENDPOINT);
            writeln!(
                out,
                "| {n} | {} | {} | {} | {} | {} | {} |",
                format_gas(d0),
                format_gas(d1),
                format_ratio(
                    self.get_net(1, i, "decodeDefault"),
                    self.get_net(0, i, "decodeDefault")
                ),
                format_gas(c0),
                format_gas(c1),
                format_ratio(
                    self.get_net(1, i, CLAIM_ENDPOINT),
                    self.get_net(0, i, CLAIM_ENDPOINT)
                ),
            )
            .unwrap();
        }

        for (v, variant) in VARIANTS.iter().enumerate() {
            writeln!(out, "\n## All endpoints, `{}` build\n", variant.label).unwrap();
            let header: Vec<&str> = all_endpoints().collect();
            writeln!(out, "| N | {} |", header.join(" | ")).unwrap();
            writeln!(out, "|---|{}", "---|".repeat(header.len())).unwrap();
            for (i, n) in self.sizes.iter().enumerate() {
                let cells: Vec<String> = self.gas[v][i].iter().map(|g| format_gas(*g)).collect();
                writeln!(out, "| {n} | {} |", cells.join(" | ")).unwrap();
            }
        }

        out
    }
}

use crate::Cfg;
use core::fmt::Debug;
use core::hash::Hash;
use specification::constants::MAX_CODE_SIZE;

/// What bytecode analysis to perform.
#[derive(Clone, Default, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AnalysisKind {
    /// Do not perform bytecode analysis.
    Raw,
    /// Perform bytecode analysis.
    #[default]
    Analyse,
}

/// EVM configuration.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct CfgEnv {
    /// Chain ID of the EVM, it will be compared to the transaction's Chain ID.
    /// Chain ID is introduced EIP-155
    pub chain_id: u64,
    /// KZG Settings for point evaluation precompile. By default, this is loaded from the ethereum mainnet trusted setup.
    #[cfg(any(feature = "c-kzg", feature = "kzg-rs"))]
    #[cfg_attr(feature = "serde", serde(skip))]
    pub kzg_settings: crate::kzg::EnvKzgSettings,
    /// Bytecode that is created with CREATE/CREATE2 is by default analysed and jumptable is created.
    /// This is very beneficial for testing and speeds up execution of that bytecode if called multiple times.
    ///
    /// Default: Analyse
    pub perf_analyse_created_bytecodes: AnalysisKind,
    /// If some it will effects EIP-170: Contract code size limit. Useful to increase this because of tests.
    /// By default it is 0x6000 (~25kb).
    pub limit_contract_code_size: Option<usize>,
    /// Skips the nonce validation against the account's nonce.
    pub disable_nonce_check: bool,
    /// A hard memory limit in bytes beyond which [crate::result::OutOfGasError::Memory] cannot be resized.
    ///
    /// In cases where the gas limit may be extraordinarily high, it is recommended to set this to
    /// a sane value to prevent memory allocation panics. Defaults to `2^32 - 1` bytes per
    /// EIP-1985.
    #[cfg(feature = "memory_limit")]
    pub memory_limit: u64,
    /// Skip balance checks if true. Adds transaction cost to balance to ensure execution doesn't fail.
    #[cfg(feature = "optional_balance_check")]
    pub disable_balance_check: bool,
    /// There are use cases where it's allowed to provide a gas limit that's higher than a block's gas limit. To that
    /// end, you can disable the block gas limit validation.
    /// By default, it is set to `false`.
    #[cfg(feature = "optional_block_gas_limit")]
    pub disable_block_gas_limit: bool,
    /// EIP-3607 rejects transactions from senders with deployed code. In development, it can be desirable to simulate
    /// calls from contracts, which this setting allows.
    /// By default, it is set to `false`.
    #[cfg(feature = "optional_eip3607")]
    pub disable_eip3607: bool,
    /// Disables all gas refunds. This is useful when using chains that have gas refunds disabled e.g. Avalanche.
    /// Reasoning behind removing gas refunds can be found in EIP-3298.
    /// By default, it is set to `false`.
    #[cfg(feature = "optional_gas_refund")]
    pub disable_gas_refund: bool,
    /// Disables base fee checks for EIP-1559 transactions.
    /// This is useful for testing method calls with zero gas price.
    /// By default, it is set to `false`.
    #[cfg(feature = "optional_no_base_fee")]
    pub disable_base_fee: bool,
}

impl CfgEnv {
    pub fn with_chain_id(mut self, chain_id: u64) -> Self {
        self.chain_id = chain_id;
        self
    }
}

impl Cfg for CfgEnv {
    fn chain_id(&self) -> u64 {
        self.chain_id
    }

    fn max_code_size(&self) -> usize {
        self.limit_contract_code_size.unwrap_or(MAX_CODE_SIZE)
    }

    fn is_eip3607_disabled(&self) -> bool {
        cfg_if::cfg_if! {
            if #[cfg(feature = "optional_eip3607")] {
                self.disable_eip3607
            } else {
                false
            }
        }
    }

    fn is_balance_check_disabled(&self) -> bool {
        cfg_if::cfg_if! {
            if #[cfg(feature = "optional_balance_check")] {
                self.disable_balance_check
            } else {
                false
            }
        }
    }

    fn is_gas_refund_disabled(&self) -> bool {
        cfg_if::cfg_if! {
            if #[cfg(feature = "optional_gas_refund")] {
                self.disable_gas_refund
            } else {
                false
            }
        }
    }

    fn is_block_gas_limit_disabled(&self) -> bool {
        cfg_if::cfg_if! {
            if #[cfg(feature = "optional_block_gas_limit")] {
                self.disable_block_gas_limit
            } else {
                false
            }
        }
    }

    fn is_nonce_check_disabled(&self) -> bool {
        self.disable_nonce_check
    }

    fn is_base_fee_check_disabled(&self) -> bool {
        cfg_if::cfg_if! {
            if #[cfg(feature = "optional_no_base_fee")] {
                self.disable_base_fee
            } else {
                false
            }
        }
    }
}

impl Default for CfgEnv {
    fn default() -> Self {
        Self {
            chain_id: 1,
            perf_analyse_created_bytecodes: AnalysisKind::default(),
            limit_contract_code_size: None,
            disable_nonce_check: false,
            #[cfg(any(feature = "c-kzg", feature = "kzg-rs"))]
            kzg_settings: crate::kzg::EnvKzgSettings::Default,
            #[cfg(feature = "memory_limit")]
            memory_limit: (1 << 32) - 1,
            #[cfg(feature = "optional_balance_check")]
            disable_balance_check: false,
            #[cfg(feature = "optional_block_gas_limit")]
            disable_block_gas_limit: false,
            #[cfg(feature = "optional_eip3607")]
            disable_eip3607: false,
            #[cfg(feature = "optional_gas_refund")]
            disable_gas_refund: false,
            #[cfg(feature = "optional_no_base_fee")]
            disable_base_fee: false,
        }
    }
}

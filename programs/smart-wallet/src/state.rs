//! State structs.

use anchor_lang::prelude::*;
use anchor_lang::solana_program;

#[account]
#[derive(Default, Debug, PartialEq)]
pub struct SmartWallet {
    /// Base used to derive.
    pub base: Pubkey,
    /// Bump seed for deriving PDA seeds.
    pub bump: u8,

    /// Minimum number of owner approvals needed to sign a transaction.
    pub threshold: u64,
    /// Minimum delay between approval and execution.
    pub minimum_delay: i64,
    /// Time after the ETA until a transaction expires.
    pub grace_period: i64,

    /// Sequence of the ownership set.
    pub owner_set_seqno: u32,
    /// Total number of [Transaction]s on this [SmartWallet].
    pub num_transactions: u64,

    /// Owners of the [SmartWallet].
    pub owners: Vec<Pubkey>,

    /// Extra space for program upgrades.
    pub reserved: [u64; 16],
}

#[account]
#[derive(Debug, Default, PartialEq)]
pub struct Transaction {
    /// The [SmartWallet] account this transaction belongs to.
    pub smart_wallet: Pubkey,
    /// The auto-incremented integer index of the transaction.
    /// All transactions on the [SmartWallet] can be looked up via this index,
    /// allowing for easier browsing of a wallet's historical transactions.
    pub index: u64,
    /// Bump seed.
    pub bump: u8,

    /// The proposer of the [Transaction].
    pub proposer: Pubkey,
    /// The instruction.
    pub instruction: Instruction,
    /// `signers[index]` is true iff `[SmartWallet]::owners[index]` signed the transaction.
    pub signers: Vec<bool>,
    /// Owner set sequence number.
    pub owner_set_seqno: u32,
    /// Estimated time transaction will be executed
    pub eta: i64,

    /// The account that executed the [Transaction].
    pub executor: Pubkey,
    /// When the transaction was executed. -1 if not executed.
    pub executed_at: i64,
}

/// Instruction.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default, PartialEq)]
pub struct Instruction {
    /// Pubkey of the instruction processor that executes this instruction
    pub program_id: Pubkey,
    /// Metadata for what accounts should be passed to the instruction processor
    pub keys: Vec<AccountMeta>,
    /// Opaque data passed to the instruction processor
    pub data: Vec<u8>,
}

impl Instruction {
    /// Space that an [Instruction] takes up.
    pub fn space(&self) -> usize {
        std::mem::size_of::<Pubkey>()
            + (self.keys.len() as usize) * std::mem::size_of::<AccountMeta>()
            + (self.data.len() as usize)
    }
}

/// Account metadata used to define Instructions
#[derive(AnchorSerialize, AnchorDeserialize, Debug, PartialEq, Copy, Clone)]
pub struct AccountMeta {
    /// An account's public key
    pub pubkey: Pubkey,
    /// True if an Instruction requires a Transaction signature matching `pubkey`.
    pub is_signer: bool,
    /// True if the `pubkey` can be loaded as a read-write account.
    pub is_writable: bool,
}

impl From<&Instruction> for solana_program::instruction::Instruction {
    fn from(tx: &Instruction) -> solana_program::instruction::Instruction {
        solana_program::instruction::Instruction {
            program_id: tx.program_id,
            accounts: tx.keys.clone().into_iter().map(Into::into).collect(),
            data: tx.data.clone(),
        }
    }
}

impl From<AccountMeta> for solana_program::instruction::AccountMeta {
    fn from(
        AccountMeta {
            pubkey,
            is_signer,
            is_writable,
        }: AccountMeta,
    ) -> solana_program::instruction::AccountMeta {
        solana_program::instruction::AccountMeta {
            pubkey,
            is_signer,
            is_writable,
        }
    }
}

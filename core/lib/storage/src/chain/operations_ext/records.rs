//! Unlike the rest `records` modules in the `storage` crate, `operations_ext::records`
//! rather consists of structures that represent database query results. This is needed
//! for employing `sqlx::query_as` macro for compile-time type checks.

// External imports
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::value::Value;
use sqlx::FromRow;
// Workspace imports
// Local imports
use crate::prover::records::ProverRun;

/// Wrapper for date and time of the first executed transaction
/// for the account.
#[derive(Debug, Serialize, Deserialize, FromRow, PartialEq)]
pub struct AccountCreatedAt {
    pub created_at: DateTime<Utc>,
}

/// A single entry from the raw response of the [`get_account_transactions_history`] query.
///
/// [`get_account_transactions_history`]: super::OperationsExtSchema::get_account_transactions_history()
#[derive(Debug, Serialize, Deserialize, FromRow, PartialEq)]
pub struct TransactionsHistoryItem {
    pub tx_id: String,
    pub hash: Option<String>,
    pub eth_block: Option<i64>,
    pub pq_id: Option<i64>,
    pub tx: Value,
    pub success: Option<bool>,
    pub fail_reason: Option<String>,
    pub commited: bool,
    pub verified: bool,
    pub created_at: DateTime<Utc>,
}

/// Stored information resulted from executing the transaction.
/// Obtained from the operations schema.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxReceiptResponse {
    pub tx_hash: String,
    pub block_number: i64,
    pub success: bool,
    pub verified: bool,
    pub fail_reason: Option<String>,
    pub prover_run: Option<ProverRun>,
}

/// Stored information resulted from executing the priority operation.
/// Obtained from the operations schema.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PriorityOpReceiptResponse {
    pub committed: bool,
    pub verified: bool,
    pub prover_run: Option<ProverRun>,
}

/// Stored executed transaction found by hash.
#[derive(Debug, Serialize, Deserialize)]
pub struct TxByHashResponse {
    pub tx_type: String,
    /// Address of transaction sender for `Transfer` and `Withdraw`.
    ///
    /// Our contract address for `Deposit`.
    pub from: String,
    /// Receiver's address for `Transfer`.
    ///
    /// Sender's address for `Deposit`.
    ///
    /// Our contract address for `Withdraw`.
    pub to: String,
    pub token: i32,
    pub amount: String,
    /// Fee paid in the zkSync network.
    /// `None` for `Deposit`.
    pub fee: Option<String>,
    pub block_number: i64,
    pub nonce: i64,
    pub created_at: String,
    pub fail_reason: Option<String>,
    pub tx: Value,
}

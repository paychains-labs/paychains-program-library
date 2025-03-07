use {
    crate::{
        error::TokenError,
        extension::{Extension, ExtensionType},
        pod::*,
    },
    bytemuck::{Pod, Zeroable},
    paychains_program::{entrypoint::ProgramResult, pubkey::Pubkey},
    paychains_zk_token_sdk::zk_token_elgamal::pod,
};

/// Confidential Transfer Extension instructions
pub mod instruction;

/// Confidential Transfer Extension processor
pub mod processor;

/// Confidential transfer mint configuration
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct ConfidentialTransferMint {
    /// Authority to modify the `ConfidentialTransferMint` configuration
    ///
    /// Note that setting an authority of `Pubkey::default()` is the idiomatic way to disable
    /// future changes to the configuration.
    pub authority: Pubkey,

    /// Indicate if newly configured accounts must be approved by the `authority` before they may be
    /// used by the user.
    ///
    /// * If `true`, no approval is required and new accounts may be used immediately
    /// * If `false`, the authority must approve newly configured accounts (see
    ///              `ConfidentialTransferInstruction::ConfigureAccount`)
    pub auto_approve_new_accounts: PodBool,

    /// * If non-zero, transfers must include ElGamal cypertext with this public key permitting the
    /// auditor to decode the transfer amount.
    /// * If all zero, auditing is currently disabled.
    pub auditor_pk: pod::ElGamalPubkey,
}

impl Extension for ConfidentialTransferMint {
    const TYPE: ExtensionType = ExtensionType::ConfidentialTransferMint;
}

/// Confidential account state
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct ConfidentialTransferAccount {
    /// `true` if this account has been approved for use. All confidential transfer operations for
    /// the account will fail until approval is granted.
    pub approved: PodBool,

    /// The public key associated with ElGamal encryption
    pub elgamal_pk: pod::ElGamalPubkey,

    /// The pending balance (encrypted by `elgamal_pk`)
    pub pending_balance: pod::ElGamalCiphertext,

    /// The available balance (encrypted by `elgamal_pk`)
    pub available_balance: pod::ElGamalCiphertext,

    /// The decryptable available balance
    pub decryptable_available_balance: pod::AeCiphertext,

    /// `pending_balance` may only be credited by `Deposit` or `Transfer` instructions if `true`
    pub allow_balance_credits: PodBool,

    /// The total number of `Deposit` and `Transfer` instructions that have credited `pending_balance`
    pub pending_balance_credit_counter: PodU64,

    /// The `expected_pending_balance_credit_counter` value that was included in the last
    /// `ApplyPendingBalance` instruction
    pub expected_pending_balance_credit_counter: PodU64,

    /// The actual `pending_balance_credit_counter` when the last `ApplyPendingBalance` instruction was executed
    pub actual_pending_balance_credit_counter: PodU64,
}

impl Extension for ConfidentialTransferAccount {
    const TYPE: ExtensionType = ExtensionType::ConfidentialTransferAccount;
}

impl ConfidentialTransferAccount {
    /// Check if a `ConfidentialTransferAccount` has been approved for use
    pub fn approved(&self) -> ProgramResult {
        if bool::from(&self.approved) {
            Ok(())
        } else {
            Err(TokenError::ConfidentialTransferAccountNotApproved.into())
        }
    }

    /// Check if a `ConfidentialTransferAccount` is in a closable state
    pub fn closable(&self) -> ProgramResult {
        if self.pending_balance == pod::ElGamalCiphertext::zeroed()
            && self.available_balance == pod::ElGamalCiphertext::zeroed()
        {
            Ok(())
        } else {
            Err(TokenError::ConfidentialTransferAccountHasBalance.into())
        }
    }
}

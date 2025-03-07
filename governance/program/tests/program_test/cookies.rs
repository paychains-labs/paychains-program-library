use paychains_program::{instruction::Instruction, pubkey::Pubkey};
use paychains_sdk::signature::Keypair;
use spl_governance::{
    addins::voter_weight::VoterWeightRecord,
    state::{
        governance::Governance, native_treasury::NativeTreasury, program_metadata::ProgramMetadata,
        proposal::ProposalV2, proposal_instruction::ProposalInstructionV2, realm::Realm,
        realm_config::RealmConfigAccount, signatory_record::SignatoryRecord,
        token_owner_record::TokenOwnerRecord, vote_record::VoteRecordV2,
    },
};

use spl_governance_test_sdk::tools::clone_keypair;

pub trait AccountCookie {
    fn get_address(&self) -> Pubkey;
}

#[derive(Debug)]
pub struct RealmCookie {
    pub address: Pubkey,

    pub account: Realm,

    pub community_mint_authority: Keypair,

    pub community_token_holding_account: Pubkey,

    pub council_mint_authority: Option<Keypair>,

    pub council_token_holding_account: Option<Pubkey>,

    pub realm_authority: Option<Keypair>,

    pub realm_config: Option<RealmConfigCookie>,
}

#[derive(Debug)]
pub struct RealmConfigCookie {
    pub address: Pubkey,
    pub account: RealmConfigAccount,
}

#[derive(Debug)]
pub struct TokenOwnerRecordCookie {
    pub address: Pubkey,

    pub account: TokenOwnerRecord,

    pub token_source: Pubkey,

    pub token_source_amount: u64,

    pub token_owner: Keypair,

    pub governance_authority: Option<Keypair>,

    pub governance_delegate: Keypair,

    pub voter_weight_record: Option<VoterWeightRecordCookie>,
}

impl TokenOwnerRecordCookie {
    pub fn get_governance_authority(&self) -> &Keypair {
        self.governance_authority
            .as_ref()
            .unwrap_or(&self.token_owner)
    }

    #[allow(dead_code)]
    pub fn clone_governance_delegate(&self) -> Keypair {
        clone_keypair(&self.governance_delegate)
    }
}

#[derive(Debug)]
pub struct GovernedProgramCookie {
    pub address: Pubkey,
    pub upgrade_authority: Keypair,
    pub data_address: Pubkey,
    pub transfer_upgrade_authority: bool,
}

impl AccountCookie for GovernedProgramCookie {
    fn get_address(&self) -> Pubkey {
        self.address
    }
}

#[derive(Debug)]
pub struct GovernedMintCookie {
    pub address: Pubkey,
    pub mint_authority: Keypair,
    pub transfer_mint_authority: bool,
}

impl AccountCookie for GovernedMintCookie {
    fn get_address(&self) -> Pubkey {
        self.address
    }
}

#[derive(Debug)]
pub struct GovernedTokenCookie {
    pub address: Pubkey,
    pub token_owner: Keypair,
    pub transfer_token_owner: bool,
    pub token_mint: Pubkey,
}

impl AccountCookie for GovernedTokenCookie {
    fn get_address(&self) -> Pubkey {
        self.address
    }
}

#[derive(Debug)]
pub struct GovernedAccountCookie {
    pub address: Pubkey,
}

impl AccountCookie for GovernedAccountCookie {
    fn get_address(&self) -> Pubkey {
        self.address
    }
}

#[derive(Debug)]
pub struct GovernanceCookie {
    pub address: Pubkey,
    pub account: Governance,
    pub next_proposal_index: u32,
}

#[derive(Debug)]
pub struct ProposalCookie {
    pub address: Pubkey,
    pub account: ProposalV2,

    pub proposal_owner: Pubkey,
}

#[derive(Debug)]
pub struct SignatoryRecordCookie {
    pub address: Pubkey,
    pub account: SignatoryRecord,
    pub signatory: Keypair,
}

#[derive(Debug)]
pub struct VoteRecordCookie {
    pub address: Pubkey,
    pub account: VoteRecordV2,
}

#[derive(Debug)]
pub struct ProposalInstructionCookie {
    pub address: Pubkey,
    pub account: ProposalInstructionV2,
    pub instruction: Instruction,
}

#[derive(Debug, Clone)]
pub struct VoterWeightRecordCookie {
    pub address: Pubkey,
    pub account: VoterWeightRecord,
}

#[derive(Debug, Clone)]
pub struct ProgramMetadataCookie {
    pub address: Pubkey,
    pub account: ProgramMetadata,
}

#[derive(Debug, Clone)]
pub struct NativeTreasuryCookie {
    pub address: Pubkey,
    pub account: NativeTreasury,
}

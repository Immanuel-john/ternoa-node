#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use ternoa_primitives::nfts::NFTId;

use codec::{Decode, Encode};
use sp_runtime::RuntimeDebug;
use sp_std::vec::Vec;
use ternoa_primitives::ternoa;

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct CapsuleData<AccountId>
where
    AccountId: Clone + Default,
{
    pub owner: AccountId,
    pub ipfs_reference: ternoa::String,
}

impl<AccountId> CapsuleData<AccountId>
where
    AccountId: Clone + Default,
{
    pub fn new(owner: AccountId, ipfs_reference: ternoa::String) -> CapsuleData<AccountId> {
        Self {
            owner,
            ipfs_reference,
        }
    }
}

impl<AccountId> Default for CapsuleData<AccountId>
where
    AccountId: Clone + Default,
{
    fn default() -> Self {
        Self {
            owner: Default::default(),
            ipfs_reference: Default::default(),
        }
    }
}

pub type CapsuleLedger<Balance> = Vec<(NFTId, Balance)>;
extern crate core;

use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::collections::LazyOption;
use near_sdk::{
    env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, Promise, PromiseOrValue, log, Balance
};
use near_sdk::json_types::U128;
use crate::nft_minter::internal_mint;

mod nft_minter;


const USUAL_DEPOSIT: Balance = 1_000_000_000_000_000_000_000_000;
const RARE_DEPOSIT: Balance = 2_000_000_000_000_000_000_000_000;
const SUPER_RARE_DEPOSIT: Balance = 3_000_000_000_000_000_000_000_000;
const MYTH_DEPOSIT: Balance = 4_000_000_000_000_000_000_000_000;
const EXCLUSIVE_DEPOSIT: Balance = 5_000_000_000_000_000_000_000_000;


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    nfts_count: u128
}

#[derive(Serialize, Deserialize, Copy, Clone)]
#[serde(crate = "near_sdk::serde")]
#[derive(BorshDeserialize, BorshSerialize)]
pub enum Rarity {
    Usual,
    Rare,
    SuperRare,
    Myth,
    Exclusive,
}

const BASE_IPFS: &str = "https://ipfs.io/ipfs/";

const USUAL_IMAGE_HASH: &str = "QmXeLAimzWpaheC1dRiZQqTXj1C5Tv4y645nTgAsD2M31A";
const RARE_IMAGE_HASH: &str = "QmWifMRCwHe4gkmdwoSrep7bSjjgeyaA9ZcLBCeyCx22KP";
const SUPER_RARE_IMAGE_HASH: &str = "QmU3oJWhg7N6Mq6cahBJ3yE7K2ECN9ugtmKFHAM1JvAWP4";
const MYTH_IMAGE_HASH: &str = "QmS7W8uPoFWoEei3mb1Unkuht2oQnKXaQaKd4JdfgxUZLE";
const EXCLUSIVE_IMAGE_HASH: &str = "QmW2UvM24fDkJ6rbhL25AbpPfAsGoMACYx8AMxxF7he3Eu";


/// accepts rarity of pack and returns image url
///
/// not tested as it's like a constant
pub(crate) fn get_pack_image(rarity: Rarity) -> String {
    match rarity {
        Rarity::Usual => format!("{}{}", BASE_IPFS, USUAL_IMAGE_HASH),
        Rarity::Rare => format!("{}{}", BASE_IPFS, RARE_IMAGE_HASH),
        Rarity::SuperRare => format!("{}{}", BASE_IPFS, SUPER_RARE_IMAGE_HASH),
        Rarity::Myth => format!("{}{}", BASE_IPFS, MYTH_IMAGE_HASH),
        Rarity::Exclusive => format!("{}{}", BASE_IPFS, EXCLUSIVE_IMAGE_HASH)
    }
}

// not tested as it's like a constant
pub(crate) fn get_pack_metadata(rarity: Rarity) -> TokenMetadata {
    let image_url = Some(get_pack_image(rarity));
    let issued_at: Option<String> = Some(env::block_timestamp().to_string());
    let title = match rarity {
        Rarity::Usual => Some("Usual pack".to_string()),
        Rarity::Rare => Some("Rare pack".to_string()),
        Rarity::SuperRare => Some("Super rare pack".to_string()),
        Rarity::Myth => Some("Myth rarity pack".to_string()),
        Rarity::Exclusive => Some("Exclusive pack".to_string())
    };
    let extra: Option<String> = Some(format!("{{\"type\": \"pack\", \"rarity\": {}}}", near_sdk::serde_json::to_string(&rarity).expect("Cannot serialize rarity")));
    TokenMetadata {
        title,
        description: None,
        media: image_url,
        media_hash: None,
        copies: None,
        issued_at,
        expires_at: None,
        starts_at: None,
        updated_at: None,
        extra,
        reference: None,
        reference_hash: None
    }
}

// not tested as it's like a constant
pub fn map_deposit_to_rarity(deposit: Balance) -> Rarity {
    match deposit {
        USUAL_DEPOSIT => Rarity::Usual,
        RARE_DEPOSIT => Rarity::Rare,
        SUPER_RARE_DEPOSIT => Rarity::SuperRare,
        MYTH_DEPOSIT => Rarity::Myth,
        EXCLUSIVE_DEPOSIT => Rarity::Exclusive,
        _ => panic!("Wrong deposit")
    }
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Approval,
    Enumeration
}

#[near_bindgen]
impl Contract {

    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        let metadata = NFTContractMetadata {
            spec: NFT_METADATA_SPEC.to_string(),
            name: "Hockey packs presale".to_string(),
            symbol: "HCM_PACKS".to_string(),
            icon: None,
            base_uri: None,
            reference: None,
            reference_hash: None
        };

        metadata.assert_valid();
        Self {
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
            nfts_count: 0
        }
    }

    #[payable]
    pub fn nft_buy_pack(
        &mut self,
        receiver_id: AccountId,
    ) -> Token {
        let token_id = format!("pack-{}", (self.nfts_count + 1).to_string());
        let token_metadata = get_pack_metadata(map_deposit_to_rarity(env::attached_deposit()));
        let nft = internal_mint(&mut self.tokens, token_id, receiver_id, Some(token_metadata));
        self.nfts_count += 1;
        nft
    }
}


near_contract_standards::impl_non_fungible_token_core!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_approval!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_enumeration!(Contract, tokens);

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;
    use std::collections::HashMap;

    use super::*;
    const MINT_STORAGE_COST: u128 = 5870000000000000000000;

    fn set_context(predecessor_account_id: AccountId, attached_deposit: Option<Balance>) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        if attached_deposit.is_some() {
            builder.attached_deposit(attached_deposit.unwrap());
        }

        testing_env!(builder.build());
        builder
    }

    #[test]
    fn test_new() {
        set_context(accounts(1), None);
        let contract = Contract::new(accounts(1).into());
        assert_eq!(contract.nft_token("1".to_string()), None);
    }

    #[test]
    #[should_panic(expected = "The contract is not initialized")]
    fn test_default() {
        let context = set_context(accounts(1), None);
        let _contract = Contract::default();
    }

    #[test]
    fn test_buy_pack() {
        let mut context = set_context(accounts(0), Some(USUAL_DEPOSIT));
        let mut contract = Contract::new(accounts(0).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance()
            .build());

        let token = contract.nft_buy_pack(accounts(0));
        assert_eq!(token.token_id, "pack-1");
        assert_eq!(token.owner_id.to_string(), accounts(0).to_string());
        assert_eq!(token.metadata.unwrap(), get_pack_metadata(Rarity::Usual));
        assert_eq!(token.approved_account_ids.unwrap(), HashMap::new());
        assert_eq!(contract.nfts_count, 1);
    }
}
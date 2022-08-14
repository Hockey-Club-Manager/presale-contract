use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_contract_standards::non_fungible_token::metadata::{
    TokenMetadata, NFT_METADATA_SPEC,
};
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_contract_standards::non_fungible_token::events::NftMint;
use near_sdk::{
    env, near_bindgen, AccountId, log, Balance
};


pub fn internal_mint(
    tokens: &mut NonFungibleToken,
    token_id: TokenId,
    token_owner_id: AccountId,
    token_metadata: Option<TokenMetadata>,
) -> Token {
    let token = tokens.internal_mint_with_refund(
        token_id,
        token_owner_id,
        token_metadata,
        None
    );
    NftMint { owner_id: &token.owner_id, token_ids: &[&token.token_id], memo: None }.emit();
    token
}


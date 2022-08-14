extern crate core;

use std::collections::HashMap;
use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC
};
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::collections::LazyOption;
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, Promise, PromiseOrValue, log, Balance, assert_one_yocto};
use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use crate::nft_minter::internal_mint;

mod nft_minter;


const USUAL_DEPOSIT: Balance = 1_000_000_000_000_000_000_000_000;
const RARE_DEPOSIT: Balance = 2_000_000_000_000_000_000_000_000;
const SUPER_RARE_DEPOSIT: Balance = 3_000_000_000_000_000_000_000_000;
const MYTH_DEPOSIT: Balance = 4_000_000_000_000_000_000_000_000;
const EXCLUSIVE_DEPOSIT: Balance = 5_000_000_000_000_000_000_000_000;
const ROYALTY_PERCENT: u128 = 10;

const ICON_URL: &str = "data:image/svg+xml,%3Csvg width='787' height='796' viewBox='0 0 787 796' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M245.864 561.5C244.364 560.5 227.141 570.838 214.364 574L199.864 577C186.317 577.896 180.941 577.683 172.364 577L152.364 575L123.864 571L93.3642 566L59.8642 558L36.3642 554H21.3642C15.904 554.883 13.5794 556.79 9.86418 561C5.62846 569.278 4.40977 572.232 4.36418 574C3.7824 579.762 3.98904 583.576 4.35044 590.246L4.36418 590.5C4.54815 603.64 5.36486 610.796 8.36418 623L11.8642 633L14.3642 638.5L19.3642 644L28.3642 649.5L37.8642 653.5L52.8642 657L68.8642 660.5L83.3642 662L97.8642 663.5C126.32 664.298 142.209 664.348 170.364 663.5L186.364 662L205.864 657L226.864 650.5L239.864 645L252.864 638.5V560C252.864 560 247.364 562.5 245.864 561.5Z' fill='black'/%3E%3Cpath d='M4.36418 590.5C4.54815 603.64 5.36486 610.796 8.36418 623L11.8642 633L14.3642 638.5L19.3642 644L28.3642 649.5L37.8642 653.5L52.8642 657L68.8642 660.5L83.3642 662L97.8642 663.5C126.32 664.298 142.209 664.348 170.364 663.5L186.364 662L205.864 657L226.864 650.5L239.864 645L252.864 638.5V560C252.864 560 247.364 562.5 245.864 561.5C244.364 560.5 227.141 570.838 214.364 574L199.864 577C186.317 577.896 180.941 577.683 172.364 577L152.364 575L123.864 571L93.3642 566L59.8642 558L36.3642 554H21.3642C15.904 554.883 13.5794 556.79 9.86418 561C5.62846 569.278 4.40977 572.232 4.36418 574C3.7824 579.762 3.98904 583.576 4.35044 590.246M4.36418 590.5L4.35044 590.246M4.36418 590.5C4.35958 590.415 4.35499 590.33 4.35044 590.246' stroke='white' stroke-width='4'/%3E%3Cpath d='M543.004 561.56C544.505 560.552 561.737 570.973 574.521 574.16L589.029 577.184C602.583 578.088 607.963 577.873 616.544 577.184L636.555 575.168L665.07 571.136L695.587 566.096L729.105 558.032L752.618 554H767.626C773.09 554.89 775.415 556.813 779.133 561.056C783.371 569.4 784.59 572.378 784.636 574.16C785.218 579.969 785.011 583.813 784.649 590.537L784.636 590.793C784.452 604.038 783.634 611.251 780.633 623.553L777.132 633.633L774.63 639.177L769.627 644.721L760.623 650.265L751.117 654.297L736.109 657.826L720.1 661.354L705.592 662.866L691.085 664.378C662.614 665.182 646.715 665.232 618.545 664.378L602.536 662.866L583.026 657.826L562.014 651.273L549.007 645.729L536 639.177V560.048C536 560.048 541.503 562.568 543.004 561.56Z' fill='black'/%3E%3Cpath d='M784.636 590.793C784.452 604.038 783.634 611.251 780.633 623.553L777.132 633.633L774.63 639.177L769.627 644.721L760.623 650.265L751.117 654.297L736.109 657.826L720.1 661.354L705.592 662.866L691.085 664.378C662.614 665.182 646.715 665.232 618.545 664.378L602.536 662.866L583.026 657.826L562.014 651.273L549.007 645.729L536 639.177V560.048C536 560.048 541.503 562.568 543.004 561.56C544.505 560.552 561.737 570.973 574.521 574.16L589.029 577.184C602.583 578.088 607.963 577.873 616.544 577.184L636.555 575.168L665.07 571.136L695.587 566.096L729.105 558.032L752.618 554H767.626C773.09 554.89 775.415 556.813 779.133 561.056C783.371 569.4 784.59 572.378 784.636 574.16C785.218 579.969 785.011 583.813 784.649 590.537M784.636 590.793L784.649 590.537M784.636 590.793C784.64 590.707 784.645 590.622 784.649 590.537' stroke='white' stroke-width='4'/%3E%3Cpath d='M588.751 81.4066L588.716 81.4533C588.671 81.5147 588.626 81.5763 588.582 81.6384L582.384 90.1302L581.337 91.5654L582.639 92.7746L610.298 118.465L611.948 119.998L613.275 118.179L620.971 107.633C621.235 107.308 621.485 106.966 621.72 106.607L621.745 106.572L622.793 105.136L622.687 105.038L665.084 35.8225C667.773 31.432 667.622 25.6137 664.69 21.4002L658.01 11.8027C653.645 5.53141 645.105 5.36913 640.555 11.5079L592.214 76.7337L592.182 76.7043L590.855 78.5229L588.751 81.4066Z' fill='black' stroke='white' stroke-width='4'/%3E%3Cpath d='M197.249 81.4066L197.284 81.4533C197.329 81.5147 197.374 81.5763 197.418 81.6384L203.616 90.1302L204.663 91.5654L203.361 92.7746L175.702 118.465L174.052 119.998L172.725 118.179L165.029 107.633C164.765 107.308 164.515 106.966 164.28 106.607L164.255 106.572L163.207 105.136L163.313 105.038L120.916 35.8225C118.227 31.432 118.378 25.6137 121.31 21.4002L127.99 11.8027C132.355 5.53141 140.895 5.36913 145.445 11.5079L193.786 76.7337L193.818 76.7043L195.145 78.5229L197.249 81.4066Z' fill='black' stroke='white' stroke-width='4'/%3E%3Cmask id='path-7-outside-1_1435_245' maskUnits='userSpaceOnUse' x='0' y='-0.197266' width='785' height='735' fill='black'%3E%3Crect fill='white' y='-0.197266' width='785' height='735'/%3E%3Cpath fill-rule='evenodd' clip-rule='evenodd' d='M17.5093 147.76C10.2029 149.217 6.61379 157.627 10.7421 163.829L380.432 719.203C386.368 728.121 399.47 728.12 405.406 719.202L775.315 163.429C779.19 157.607 775.826 149.713 768.968 148.346V148.346C645.009 123.634 544.984 71.204 509.266 17.9186C505.979 13.0154 500.726 9.64883 494.828 9.39142V9.39142C489.125 9.14248 483.779 11.8733 480.082 16.2227C459.461 40.4784 427.841 56 392.392 56C356.327 56 324.226 39.9347 303.638 14.9512C300.425 11.0516 295.691 8.61741 290.642 8.81379V8.81379C285.396 9.01782 280.717 11.9976 277.838 16.3873C242.675 69.9971 142.201 122.902 17.5093 147.76V147.76Z'/%3E%3C/mask%3E%3Cpath fill-rule='evenodd' clip-rule='evenodd' d='M17.5093 147.76C10.2029 149.217 6.61379 157.627 10.7421 163.829L380.432 719.203C386.368 728.121 399.47 728.12 405.406 719.202L775.315 163.429C779.19 157.607 775.826 149.713 768.968 148.346V148.346C645.009 123.634 544.984 71.204 509.266 17.9186C505.979 13.0154 500.726 9.64883 494.828 9.39142V9.39142C489.125 9.14248 483.779 11.8733 480.082 16.2227C459.461 40.4784 427.841 56 392.392 56C356.327 56 324.226 39.9347 303.638 14.9512C300.425 11.0516 295.691 8.61741 290.642 8.81379V8.81379C285.396 9.01782 280.717 11.9976 277.838 16.3873C242.675 69.9971 142.201 122.902 17.5093 147.76V147.76Z' fill='%238DCFF6'/%3E%3Cpath d='M380.432 719.203L373.772 723.636L373.772 723.636L380.432 719.203ZM405.406 719.202L412.065 723.635L412.065 723.635L405.406 719.202ZM277.838 16.3873L284.527 20.775L277.838 16.3873ZM480.082 16.2227L473.987 11.0411L480.082 16.2227ZM303.638 14.9512L297.464 20.0388L303.638 14.9512ZM775.315 163.429L768.655 158.997L775.315 163.429ZM509.266 17.9186L502.621 22.373L509.266 17.9186ZM4.08258 168.261L373.772 723.636L387.091 714.77L17.4016 159.396L4.08258 168.261ZM373.772 723.636C382.875 737.31 402.964 737.309 412.065 723.635L398.746 714.769C395.976 718.931 389.862 718.931 387.091 714.77L373.772 723.636ZM412.065 723.635L781.975 167.862L768.655 158.997L398.746 714.769L412.065 723.635ZM770.532 140.501C709.176 128.269 653.888 109.202 609.652 86.7157C565.237 64.1387 532.679 38.479 515.911 13.4642L502.621 22.373C521.571 50.6435 556.885 77.8413 602.402 100.979C648.097 124.207 704.8 143.711 767.404 156.192L770.532 140.501ZM473.987 11.0411C454.879 33.5169 425.484 48 392.392 48V64C430.199 64 464.043 47.44 486.177 21.4044L473.987 11.0411ZM392.392 48C358.725 48 328.885 33.0092 309.812 9.86359L297.464 20.0388C319.567 46.8601 353.929 64 392.392 64V48ZM271.148 11.9997C254.663 37.1336 222.159 62.9769 177.608 85.7309C133.243 108.39 77.664 127.61 15.9452 139.915L19.0734 155.606C82.0468 143.052 139.05 123.39 184.886 99.98C230.536 76.6646 265.85 49.2507 284.527 20.775L271.148 11.9997ZM290.331 0.819835C282.169 1.13728 275.253 5.74132 271.148 11.9997L284.527 20.775C286.181 18.2538 288.623 16.8984 290.953 16.8077L290.331 0.819835ZM495.177 1.39903C486.631 1.02598 479.022 5.11793 473.987 11.0411L486.177 21.4044C488.537 18.6286 491.62 17.259 494.48 17.3838L495.177 1.39903ZM309.812 9.86359C305.235 4.30974 298.212 0.513327 290.331 0.819835L290.953 16.8077C293.171 16.7215 295.614 17.7935 297.464 20.0388L309.812 9.86359ZM781.975 167.862C789.073 157.197 782.852 142.957 770.532 140.501L767.404 156.192C768.8 156.47 769.307 158.018 768.655 158.997L781.975 167.862ZM515.911 13.4642C511.423 6.76905 503.98 1.78326 495.177 1.39903L494.48 17.3838C497.471 17.5144 500.535 19.2617 502.621 22.373L515.911 13.4642ZM17.4016 159.396C16.5015 158.043 17.2212 155.975 19.0734 155.606L15.9452 139.915C3.18466 142.459 -3.27395 157.21 4.08258 168.261L17.4016 159.396Z' fill='white' mask='url(%23path-7-outside-1_1435_245)'/%3E%3Cpath d='M36 159V379C36 383.418 39.5817 387 44 387H89.8844C94.3026 387 97.8844 383.418 97.8844 379V298.917C97.8844 297.812 98.7798 296.917 99.8843 296.917H243.116C244.22 296.917 245.116 297.812 245.116 298.917V379C245.116 383.418 248.697 387 253.116 387H299C303.418 387 307 383.418 307 379V159C307 154.582 303.418 151 299 151H253.116C248.697 151 245.116 154.582 245.116 159V239.083C245.116 240.188 244.22 241.083 243.116 241.083H99.8844C98.7798 241.083 97.8844 240.188 97.8844 239.083V159C97.8844 154.582 94.3026 151 89.8844 151H44C39.5817 151 36 154.582 36 159Z' fill='white' stroke='black' stroke-width='6'/%3E%3Cpath d='M505.914 380.254L505.93 380.263L505.947 380.272C514.581 384.769 524.079 387 534.387 387H735C739.418 387 743 383.418 743 379V339.167C743 334.748 739.418 331.167 735 331.167H548.816C545.487 331.167 543.258 330.451 541.794 329.361C540.641 328.144 539.912 326.354 539.912 323.625V214.375C539.912 211.312 540.703 209.633 541.725 208.691C543.188 207.572 545.435 206.833 548.816 206.833H735C739.418 206.833 743 203.252 743 198.833V159C743 154.582 739.418 151 735 151H534.387C524.079 151 514.581 153.231 505.947 157.728L505.93 157.737L505.914 157.746C497.565 162.226 490.819 168.302 485.727 175.962C480.575 183.497 478 191.965 478 201.278V336.722C478 346.041 480.578 354.605 485.716 362.34L485.727 362.358L485.739 362.375C490.835 369.822 497.58 375.782 505.914 380.254Z' fill='white' stroke='black' stroke-width='6'/%3E%3Cpath d='M244 435V656C244 660.418 247.582 664 252 664H297.823C302.241 664 305.823 660.418 305.823 656V518.905C305.823 517.097 308.031 516.217 309.275 517.529L386.679 599.223C389.843 602.561 395.163 602.552 398.314 599.201L475.36 517.282C476.601 515.962 478.817 516.84 478.817 518.652V656C478.817 660.418 482.399 664 486.817 664H533C537.418 664 541 660.418 541 656V435C541 430.582 537.418 427 533 427H482.182C479.971 427 477.858 427.915 476.346 429.529L393.952 517.427C393.163 518.269 391.827 518.27 391.036 517.43L308.289 429.517C306.777 427.911 304.669 427 302.464 427H252C247.582 427 244 430.582 244 435Z' fill='white' stroke='black' stroke-width='6'/%3E%3Cpath d='M392.5 397L342.704 263.5L442.296 263.5L392.5 397Z' fill='black'/%3E%3Cpath fill-rule='evenodd' clip-rule='evenodd' d='M380.383 134L343 263.5H442.593L405.209 134H380.383Z' fill='black'/%3E%3Cpath fill-rule='evenodd' clip-rule='evenodd' d='M363.183 68L344 87.538L380.155 125.219H393H405.845L442 87.538L422.817 68C413.485 70.5797 403.451 71.9825 393 71.9825C382.549 71.9825 372.515 70.5797 363.183 68Z' fill='black'/%3E%3Cpath d='M393 792C406.391 792 418.693 788.349 427.762 782.237C436.807 776.142 443 767.294 443 757C443 746.706 436.807 737.858 427.762 731.763C418.693 725.651 406.391 722 393 722C379.609 722 367.307 725.651 358.238 731.763C349.193 737.858 343 746.706 343 757C343 767.294 349.193 776.142 358.238 782.237C367.307 788.349 379.609 792 393 792Z' fill='black' stroke='white' stroke-width='8'/%3E%3Cpath d='M393 766C406.391 766 418.693 762.349 427.762 756.237C436.807 750.142 443 741.294 443 731C443 720.706 436.807 711.858 427.762 705.763C418.693 699.651 406.391 696 393 696C379.609 696 367.307 699.651 358.238 705.763C349.193 711.858 343 720.706 343 731C343 741.294 349.193 750.142 358.238 756.237C367.307 762.349 379.609 766 393 766Z' fill='black' stroke='white' stroke-width='8'/%3E%3C/svg%3E%0A";

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Payout {
    pub payout: HashMap<AccountId, U128>,
}


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
            icon: Some(ICON_URL.to_string()),
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

    #[allow(unused_variables)]
    pub fn nft_payout(
        &self,
        token_id: TokenId,
        balance: U128,
        max_len_payout: u32
    ) -> Payout {
        let mut payout: Payout = Payout { payout: HashMap::new() };
        let balance_u128: u128 = balance.into();
        let owner_id = self.tokens.owner_by_id.get(&token_id).expect("Token id does not exist");
        payout.payout.insert(env::current_account_id(), U128::from(balance_u128 / 100 * ROYALTY_PERCENT));
        payout.payout.insert(owner_id, U128::from(balance_u128 / 100  * (100 - ROYALTY_PERCENT)));
        payout
    }

    #[payable]
    #[allow(unused_variables)]
    pub fn nft_transfer_payout(
        &mut self,
        receiver_id: AccountId,
        token_id: String,
        approval_id: u64,
        balance: U128,
        max_len_payout: u32) -> Payout {
        assert_one_yocto();

        let owner_id = self.tokens.owner_by_id.get(&token_id).expect("Token id does not exist");
        self.tokens.nft_transfer(receiver_id.clone(), token_id.clone(), Some(approval_id), None);

        let balance_u128: u128 = balance.into();
        let mut payout: Payout = Payout { payout: HashMap::new() };
        payout.payout.insert(env::current_account_id(), U128::from(balance_u128 / 100 * ROYALTY_PERCENT));
        payout.payout.insert(owner_id, U128::from(balance_u128 / 100  * (100 - ROYALTY_PERCENT)));
        payout

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
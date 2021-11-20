/*!
A stub contract that implements nft_on_approve for simulation testing nft_approve.
*/
use near_contract_standards::non_fungible_token::approval::NonFungibleTokenApprovalReceiver;
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::ValidAccountId;
use near_sdk::{
    env, ext_contract, log, near_bindgen, setup_alloc, AccountId, Balance, Gas, PanicOnDefault,
    PromiseOrValue,
};

setup_alloc!();

// TODO: Stubbed interface:
// * receive NFT
// * transfer NFT
// * get supported NFTs
// * get NFTs owned
// * on approve NFT (Needed?)

const BASE_GAS: Gas = 5_000_000_000_000;
const PROMISE_CALL: Gas = 5_000_000_000_000;
const GAS_FOR_NFT_ON_APPROVE: Gas = BASE_GAS + PROMISE_CALL;

const NO_DEPOSIT: Balance = 0;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct ApprovalReceiver {
    non_fungible_token_account_id: AccountId,
}

// Defining cross-contract interface. This allows to create a new promise.
#[ext_contract(ext_self)]
pub trait ValueReturnTrait {
    fn ok_go(&self, msg: String) -> PromiseOrValue<String>;
}

// Have to repeat the same trait for our own implementation.
trait ValueReturnTrait {
    fn ok_go(&self, msg: String) -> PromiseOrValue<String>;
}

#[near_bindgen]
impl ApprovalReceiver {
    #[init]
    pub fn new(non_fungible_token_account_id: ValidAccountId) -> Self {
        Self { non_fungible_token_account_id: non_fungible_token_account_id.into() }
    }
}

#[near_bindgen]
impl NonFungibleTokenApprovalReceiver for ApprovalReceiver {
    /// Could do anything useful to the approval-receiving contract, such as store the given
    /// approval_id for use later when calling the NFT contract. Can also return whatever it wants,
    /// maybe after further promise calls. This one simulates "return anything" behavior only.
    /// Supports the following `msg` patterns:
    /// * "return-now" - immediately return `"cool"`
    /// * anything else - return the given `msg` after one more cross-contract call
    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId,
        approval_id: u64,
        msg: String,
    ) -> PromiseOrValue<String> {
        // Verifying that we were called by non-fungible token contract that we expect.
        assert_eq!(
            &env::predecessor_account_id(),
            &self.non_fungible_token_account_id,
            "Only supports the one non-fungible token contract"
        );
        log!(
            "in nft_on_approve; sender_id={}, previous_owner_id={}, token_id={}, msg={}",
            &token_id,
            &owner_id,
            &approval_id,
            msg
        );
        match msg.as_str() {
            "return-now" => PromiseOrValue::Value("cool".to_string()),
            _ => {
                let prepaid_gas = env::prepaid_gas();
                let account_id = env::current_account_id();
                ext_self::ok_go(msg, &account_id, NO_DEPOSIT, prepaid_gas - GAS_FOR_NFT_ON_APPROVE)
                    .into()
            }
        }
    }
}

#[near_bindgen]
impl ValueReturnTrait for ApprovalReceiver {
    fn ok_go(&self, msg: String) -> PromiseOrValue<String> {
        log!("in ok_go, msg={}", msg);
        PromiseOrValue::Value(msg)
    }
}

// /*!
// A stub contract that implements nft_on_transfer for simulation testing nft_transfer_call.
// */
// use near_contract_standards::non_fungible_token::core::NonFungibleTokenReceiver;
// use near_contract_standards::non_fungible_token::TokenId;
// use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
// use near_sdk::json_types::ValidAccountId;
// use near_sdk::{
//     env, ext_contract, log, near_bindgen, setup_alloc, AccountId, Balance, Gas, PanicOnDefault,
//     PromiseOrValue,
// };

// setup_alloc!();

// const BASE_GAS: Gas = 5_000_000_000_000;
// const PROMISE_CALL: Gas = 5_000_000_000_000;
// const GAS_FOR_NFT_ON_TRANSFER: Gas = BASE_GAS + PROMISE_CALL;

// const NO_DEPOSIT: Balance = 0;

// #[near_bindgen]
// #[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
// pub struct TokenReceiver {
//     non_fungible_token_account_id: AccountId,
// }

// // Defining cross-contract interface. This allows to create a new promise.
// #[ext_contract(ext_self)]
// pub trait ValueReturnTrait {
//     fn ok_go(&self, return_it: bool) -> PromiseOrValue<bool>;
// }

// // Have to repeat the same trait for our own implementation.
// trait ValueReturnTrait {
//     fn ok_go(&self, return_it: bool) -> PromiseOrValue<bool>;
// }

// #[near_bindgen]
// impl TokenReceiver {
//     #[init]
//     pub fn new(non_fungible_token_account_id: ValidAccountId) -> Self {
//         Self { non_fungible_token_account_id: non_fungible_token_account_id.into() }
//     }
// }

// #[near_bindgen]
// impl NonFungibleTokenReceiver for TokenReceiver {
//     /// Returns true if token should be returned to `sender_id`
//     /// Four supported `msg`s:
//     /// * "return-it-now" - immediately return `true`
//     /// * "keep-it-now" - immediately return `false`
//     /// * "return-it-later" - make cross-contract call which resolves with `true`
//     /// * "keep-it-later" - make cross-contract call which resolves with `false`
//     /// Otherwise panics, which should also return token to `sender_id`
//     fn nft_on_transfer(
//         &mut self,
//         sender_id: AccountId,
//         previous_owner_id: AccountId,
//         token_id: TokenId,
//         msg: String,
//     ) -> PromiseOrValue<bool> {
//         // Verifying that we were called by non-fungible token contract that we expect.
//         assert_eq!(
//             &env::predecessor_account_id(),
//             &self.non_fungible_token_account_id,
//             "Only supports the one non-fungible token contract"
//         );
//         log!(
//             "in nft_on_transfer; sender_id={}, previous_owner_id={}, token_id={}, msg={}",
//             &sender_id,
//             &previous_owner_id,
//             &token_id,
//             msg
//         );
//         match msg.as_str() {
//             "return-it-now" => PromiseOrValue::Value(true),
//             "return-it-later" => {
//                 let prepaid_gas = env::prepaid_gas();
//                 let account_id = env::current_account_id();
//                 ext_self::ok_go(
//                     true,
//                     &account_id,
//                     NO_DEPOSIT,
//                     prepaid_gas - GAS_FOR_NFT_ON_TRANSFER,
//                 )
//                 .into()
//             }
//             "keep-it-now" => PromiseOrValue::Value(false),
//             "keep-it-later" => {
//                 let prepaid_gas = env::prepaid_gas();
//                 let account_id = env::current_account_id();
//                 ext_self::ok_go(
//                     false,
//                     &account_id,
//                     NO_DEPOSIT,
//                     prepaid_gas - GAS_FOR_NFT_ON_TRANSFER,
//                 )
//                 .into()
//             }
//             _ => env::panic(b"unsupported msg"),
//         }
//     }
// }

// #[near_bindgen]
// impl ValueReturnTrait for TokenReceiver {
//     fn ok_go(&self, return_it: bool) -> PromiseOrValue<bool> {
//         log!("in ok_go, return_it={}", return_it);
//         PromiseOrValue::Value(return_it)
//     }
// }


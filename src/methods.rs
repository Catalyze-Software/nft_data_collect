use candid::{candid_method, Principal};
use ic_cdk::{caller, storage};
use ic_cdk_macros::{post_upgrade, pre_upgrade, query, update};

use crate::{
    logic::store::{Store, Wallets, DATA},
    rust_declarations::ext_declaration::{AccountIdentifier__1, TokenIndex},
};

#[pre_upgrade]
pub fn pre_upgrade() {
    DATA.with(|data| storage::stable_save((&*data.borrow(),)).unwrap());
}

#[post_upgrade]
pub fn post_upgrade() {
    let (old_store,): (Store,) = storage::stable_restore().unwrap();
    DATA.with(|data| *data.borrow_mut() = old_store);
}

#[update]
#[candid_method(update)]
async fn add_to_whitelist(nns_principal: Principal) -> Result<Vec<u32>, String> {
    Store::add_to_whitelist(caller(), nns_principal).await
}

#[query]
#[candid_method(query)]
async fn get_canister() -> String {
    Store::get_canister()
}

#[query]
#[candid_method(query)]
async fn get_snapshot() -> Vec<(TokenIndex, AccountIdentifier__1)> {
    Store::get_snapshot()
}

#[update]
#[candid_method(update)]
async fn take_snapshot(canister_id: String) -> Result<(), String> {
    Store::take_snapshot(canister_id).await
}

#[query]
#[candid_method(query)]
async fn get_whitelist() -> Vec<(u32, Wallets)> {
    Store::get_whitelist()
}

// Hacky way to expose the candid interface to the outside world
#[query(name = "__get_candid_interface_tmp_hack")]
#[candid_method(query, rename = "__get_candid_interface_tmp_hack")]
pub fn __export_did_tmp_() -> String {
    use candid::export_service;
    export_service!();
    __export_service()
}

// Method used to save the candid interface to a file
#[test]
pub fn candid() {
    use std::env;
    use std::fs::write;
    use std::path::PathBuf;

    let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let dir = dir.parent().unwrap().join("candid");
    write(
        dir.join(format!("nft_data_collect.did")),
        __export_did_tmp_(),
    )
    .expect("Write failed.");
}

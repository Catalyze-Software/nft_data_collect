use candid::{candid_method, Principal};
use ic_cdk::{caller, storage};
use ic_cdk_macros::{post_upgrade, pre_upgrade, query, update};

use crate::logic::store::{Store, Wallets, DATA};

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
async fn get_whitelist() -> Result<Vec<(u32, Wallets)>, String> {
    if caller().to_string() == "ledm3-52ncq-rffuv-6ed44-hg5uo-iicyu-pwkzj-syfva-heo4k-p7itq-aqe"
        || caller().to_string() == "ve3v4-o7xuv-ijejl-vcyfx-hjy3b-owwtx-jte2k-2bciw-spskd-jgmvd-rqe"
    {
        return Ok(Store::get_whitelist());
    }

    return Err("Unauthorized".to_string());
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

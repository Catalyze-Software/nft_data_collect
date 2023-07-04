use candid::candid_method;
use ic_cdk::{caller, storage};
use ic_cdk_macros::{post_upgrade, pre_upgrade, query, update};

use crate::logic::store::{Store, DATA};

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
async fn add_to_whitelist() -> Result<Vec<u32>, String> {
    Store::add_to_whitelist(caller()).await
}

#[query]
#[candid_method(query)]
async fn get_whitelist() -> Result<Vec<(u32, String)>, String> {
    if caller().to_string() != "ledm3-52ncq-rffuv-6ed44-hg5uo-iicyu-pwkzj-syfva-heo4k-p7itq-aqe" {
        return Err("Unauthorized".to_string());
    }

    Ok(Store::get_whitelist())
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

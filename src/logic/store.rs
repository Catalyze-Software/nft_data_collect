use std::{cell::RefCell, collections::HashMap, time::Duration};

use candid::{CandidType, Principal};

use ic_ledger_types::{AccountIdentifier, Subaccount};
use serde::Deserialize;

use crate::rust_declarations::ext_declaration::{AccountIdentifier__1, TokenIndex, SERVICE};

pub static DAY_IN_NANOS: u64 = Duration::from_secs(1 * 24 * 60 * 60).as_nanos() as u64;

#[derive(Deserialize, CandidType, Clone)]
pub struct Wallets {
    pub nft_wallet: Principal,
    pub nns_wallet: Principal,
}

#[derive(Deserialize, CandidType, Clone)]
pub struct Store {
    pub whitelist: HashMap<u32, Wallets>,
    pub canister_id: String,
    pub snapshot: Vec<(TokenIndex, AccountIdentifier__1)>,
}

impl Default for Store {
    fn default() -> Self {
        Self {
            whitelist: HashMap::new(),
            canister_id: Default::default(),
            snapshot: Default::default(),
        }
    }
}

thread_local! {
    pub static DATA: RefCell<Store>  = RefCell::new(Store::default());
}

impl Store {
    pub async fn add_to_whitelist(
        caller: Principal,
        nns_principal: Principal,
    ) -> Result<Vec<u32>, String> {
        let identifier = Self::principal_to_identifier(&caller);

        let registry = DATA.with(|d| d.borrow().snapshot.clone());

        if registry.len() == 0 {
            return Err("No snapshot taken".to_string());
        }

        let filtered_data = registry
            .iter()
            .filter(|(_, _account_identifier)| _account_identifier == &identifier.to_string())
            .map(|f| f.0)
            .collect::<Vec<u32>>();

        if filtered_data.len() == 0 {
            return Err("No NFTs found for this principal".to_string());
        }

        for _mint_id in &filtered_data {
            DATA.with(|data| {
                data.borrow_mut().whitelist.insert(
                    *_mint_id,
                    Wallets {
                        nft_wallet: caller,
                        nns_wallet: nns_principal,
                    },
                );
            });
        }
        Ok(filtered_data)
    }

    pub fn get_whitelist() -> Vec<(u32, Wallets)> {
        DATA.with(|data| {
            data.borrow()
                .whitelist
                .iter()
                .map(|d| (d.0.clone(), d.1.clone()))
                .collect::<Vec<(u32, Wallets)>>()
        })
    }

    pub fn principal_to_identifier(principal: &Principal) -> String {
        let identifier = AccountIdentifier::new(principal, &Subaccount([0; 32]));
        identifier.to_string()
    }

    pub fn get_canister() -> String {
        DATA.with(|data| data.borrow().canister_id.to_string())
    }

    pub fn get_snapshot() -> Vec<(TokenIndex, AccountIdentifier__1)> {
        DATA.with(|data| data.borrow().snapshot.clone())
    }

    pub async fn take_snapshot(canister_id: String) -> Result<(), String> {
        if &canister_id == "" {
            return Err("Canister ID not set".to_string());
        }

        let ext = SERVICE(Principal::from_text(&canister_id).unwrap());

        let snapshot = ext.getRegistry().await;

        DATA.with(|data| match snapshot {
            Ok((_registry_data,)) => {
                data.borrow_mut().canister_id = canister_id;
                data.borrow_mut().snapshot = _registry_data;
                Ok(())
            }
            Err(err) => Err(err.1),
        })
    }
}

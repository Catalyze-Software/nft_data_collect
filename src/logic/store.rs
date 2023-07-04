use std::{cell::RefCell, collections::HashMap, time::Duration};

use candid::{CandidType, Principal};

use ic_ledger_types::{AccountIdentifier, Subaccount};
use serde::Deserialize;

use crate::rust_declarations::ext_declaration::SERVICE;

pub static DAY_IN_NANOS: u64 = Duration::from_secs(1 * 24 * 60 * 60).as_nanos() as u64;

#[derive(Deserialize, CandidType, Clone)]
pub struct Store {
    pub whitelist: HashMap<u32, Principal>,
}

impl Default for Store {
    fn default() -> Self {
        Self {
            whitelist: HashMap::new(),
        }
    }
}

thread_local! {
    pub static DATA: RefCell<Store>  = RefCell::new(Store::default());
}

impl Store {
    pub async fn add_to_whitelist(caller: Principal) -> Result<Vec<u32>, String> {
        let ext = SERVICE(Principal::from_text("gksxm-waaaa-aaaao-aapjq-cai").unwrap());

        let identifier = Self::principal_to_identifier(&caller);
        let registry = ext.getRegistry().await;

        match registry {
            Ok((_registry_data,)) => {
                let filtered_data = _registry_data
                    .iter()
                    .filter(|(_, _account_identifier)| {
                        _account_identifier == &identifier.to_string()
                    })
                    .map(|f| f.0)
                    .collect::<Vec<u32>>();

                if filtered_data.len() == 0 {
                    return Err("No NFTs found for this principal".to_string());
                }

                for _mint_id in &filtered_data {
                    DATA.with(|data| {
                        data.borrow_mut().whitelist.insert(*_mint_id, caller);
                    });
                }
                Ok(filtered_data)
            }
            Err(err) => Err(err.1),
        }
    }

    pub fn get_whitelist() -> Vec<(u32, String)> {
        DATA.with(|data| {
            data.borrow()
                .whitelist
                .iter()
                .map(|d| (d.0.clone(), d.1.to_string()))
                .collect::<Vec<(u32, String)>>()
        })
    }

    pub fn principal_to_identifier(principal: &Principal) -> String {
        let identifier = AccountIdentifier::new(principal, &Subaccount([0; 32]));
        identifier.to_string()
    }
}

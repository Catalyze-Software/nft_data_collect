// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
use candid::{self, CandidType, Deserialize, Principal};
use ic_cdk::api::call::CallResult as Result;

#[derive(CandidType, Deserialize)]
pub struct File {
    data: Vec<serde_bytes::ByteBuf>,
    ctype: String,
}

#[derive(CandidType, Deserialize)]
pub struct Asset {
    thumbnail: Option<File>,
    name: String,
    payload: File,
}

#[allow(non_camel_case_types)]
pub type SubAccount__1 = serde_bytes::ByteBuf;
pub type TokenIndex = u32;
#[allow(non_camel_case_types)]
pub type AccountIdentifier__1 = String;
#[derive(CandidType, Deserialize)]
pub struct Settlement {
    subaccount: SubAccount__1,
    seller: Principal,
    buyer: AccountIdentifier__1,
    price: u64,
}

pub type TokenIdentifier = String;
pub type AccountIdentifier = String;
#[derive(CandidType, Deserialize)]
#[allow(non_camel_case_types)]
pub enum User {
    principal(Principal),
    address(AccountIdentifier),
}

#[derive(CandidType, Deserialize)]
pub struct BalanceRequest {
    token: TokenIdentifier,
    user: User,
}

pub type Balance = candid::Nat;
#[derive(CandidType, Deserialize)]
#[allow(non_camel_case_types)]
pub enum CommonError__1 {
    InvalidToken(TokenIdentifier),
    Other(String),
}

#[derive(CandidType, Deserialize)]
#[allow(non_camel_case_types)]
pub enum BalanceResponse {
    ok(Balance),
    err(CommonError__1),
}
#[allow(non_camel_case_types)]
pub type TokenIdentifier__1 = String;
#[derive(CandidType, Deserialize)]
pub enum CommonError {
    InvalidToken(TokenIdentifier),
    Other(String),
}

#[derive(CandidType, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Result_6 {
    ok(AccountIdentifier__1),
    err(CommonError),
}

pub type Time = candid::Int;
#[derive(CandidType, Deserialize)]
pub struct Listing {
    locked: Option<Time>,
    seller: Principal,
    price: u64,
}

#[derive(CandidType, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Result_7 {
    ok(AccountIdentifier__1, Option<Listing>, Vec<u32>),
    err(CommonError),
}

pub type Extension = String;
#[derive(CandidType, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Metadata {
    fungible {
        decimals: u8,
        metadata: Option<serde_bytes::ByteBuf>,
        name: String,
        symbol: String,
    },
    nonfungible {
        metadata: Option<serde_bytes::ByteBuf>,
    },
}

#[derive(CandidType, Deserialize)]
pub struct HeaderField(String, String);

#[derive(CandidType, Deserialize)]
pub struct HttpRequest {
    url: String,
    method: String,
    body: serde_bytes::ByteBuf,
    headers: Vec<HeaderField>,
}

#[derive(CandidType, Deserialize)]
pub struct HttpStreamingCallbackToken {
    key: String,
    sha256: Option<serde_bytes::ByteBuf>,
    index: candid::Nat,
    content_encoding: String,
}

#[derive(CandidType, Deserialize)]
pub struct HttpStreamingCallbackResponse {
    token: Option<HttpStreamingCallbackToken>,
    body: serde_bytes::ByteBuf,
}

#[derive(CandidType, Deserialize)]
pub struct HttpResponse {
    body: serde_bytes::ByteBuf,
    headers: Vec<HeaderField>,
    streaming_strategy: Option<bool>,
    status_code: u16,
}

#[derive(CandidType, Deserialize)]
pub struct ListRequest {
    token: TokenIdentifier__1,
    from_subaccount: Option<SubAccount__1>,
    price: Option<u64>,
}

#[derive(CandidType, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Result_3 {
    ok,
    err(CommonError),
}

#[derive(CandidType, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Result_5 {
    ok(Metadata),
    err(CommonError),
}

#[derive(CandidType, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Result_4 {
    ok(String),
    err(CommonError),
}

#[allow(non_camel_case_types)]
pub type Balance__1 = candid::Nat;

#[derive(CandidType, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Result_2 {
    ok(Balance__1),
    err(CommonError),
}

#[derive(CandidType, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Result_1 {
    ok(Vec<TokenIndex>),
    err(CommonError),
}

#[derive(CandidType, Deserialize)]
#[allow(non_camel_case_types)]
pub enum MotokoResult {
    ok(Vec<(TokenIndex, Option<Listing>, Option<serde_bytes::ByteBuf>)>),
    err(CommonError),
}

#[derive(CandidType, Deserialize)]
pub struct Transaction {
    token: TokenIdentifier__1,
    time: Time,
    seller: Principal,
    buyer: AccountIdentifier__1,
    price: u64,
}

pub type Memo = serde_bytes::ByteBuf;
pub type SubAccount = serde_bytes::ByteBuf;
#[derive(CandidType, Deserialize)]
pub struct TransferRequest {
    to: User,
    token: TokenIdentifier,
    notify: bool,
    from: User,
    memo: Memo,
    subaccount: Option<SubAccount>,
    amount: Balance,
}

#[derive(CandidType, Deserialize)]
#[allow(non_camel_case_types)]
pub enum TransferResponse_err {
    CannotNotify(AccountIdentifier),
    InsufficientBalance,
    InvalidToken(TokenIdentifier),
    Rejected,
    Unauthorized(AccountIdentifier),
    Other(String),
}

#[derive(CandidType, Deserialize)]
#[allow(non_camel_case_types)]
pub enum TransferResponse {
    ok(Balance),
    err(TransferResponse_err),
}

pub struct SERVICE(pub Principal);
#[allow(non_snake_case)]
impl SERVICE {
    pub async fn acceptCycles(&self) -> Result<()> {
        ic_cdk::call(self.0, "acceptCycles", ()).await
    }
    pub async fn addAsset(&self, arg0: Asset) -> Result<(candid::Nat,)> {
        ic_cdk::call(self.0, "addAsset", (arg0,)).await
    }
    pub async fn addAssetAtIndex(&self, arg0: Asset, arg1: candid::Nat) -> Result<(candid::Nat,)> {
        ic_cdk::call(self.0, "addAssetAtIndex", (arg0, arg1)).await
    }
    pub async fn adminKillHeartbeat(&self) -> Result<()> {
        ic_cdk::call(self.0, "adminKillHeartbeat", ()).await
    }
    pub async fn adminRemoveListing(&self, arg0: candid::Nat) -> Result<()> {
        ic_cdk::call(self.0, "adminRemoveListing", (arg0,)).await
    }
    pub async fn adminStartHeartbeat(&self) -> Result<()> {
        ic_cdk::call(self.0, "adminStartHeartbeat", ()).await
    }
    pub async fn allPayments(&self) -> Result<(Vec<(Principal, Vec<SubAccount__1>)>,)> {
        ic_cdk::call(self.0, "allPayments", ()).await
    }
    pub async fn allSettlements(&self) -> Result<(Vec<(TokenIndex, Settlement)>,)> {
        ic_cdk::call(self.0, "allSettlements", ()).await
    }
    pub async fn availableCycles(&self) -> Result<(candid::Nat,)> {
        ic_cdk::call(self.0, "availableCycles", ()).await
    }
    pub async fn balance(&self, arg0: BalanceRequest) -> Result<(BalanceResponse,)> {
        ic_cdk::call(self.0, "balance", (arg0,)).await
    }
    pub async fn bearer(&self, arg0: TokenIdentifier__1) -> Result<(Result_6,)> {
        ic_cdk::call(self.0, "bearer", (arg0,)).await
    }
    pub async fn clearPayments(&self, arg0: Principal, arg1: Vec<SubAccount__1>) -> Result<()> {
        ic_cdk::call(self.0, "clearPayments", (arg0, arg1)).await
    }
    pub async fn cronDisbursements(&self) -> Result<()> {
        ic_cdk::call(self.0, "cronDisbursements", ()).await
    }
    pub async fn cronSettlements(&self) -> Result<()> {
        ic_cdk::call(self.0, "cronSettlements", ()).await
    }
    pub async fn details(&self, arg0: TokenIdentifier__1) -> Result<(Result_7,)> {
        ic_cdk::call(self.0, "details", (arg0,)).await
    }
    pub async fn extensions(&self) -> Result<(Vec<Extension>,)> {
        ic_cdk::call(self.0, "extensions", ()).await
    }
    pub async fn getMinter(&self) -> Result<(Principal,)> {
        ic_cdk::call(self.0, "getMinter", ()).await
    }
    pub async fn getMyPrincipal(&self) -> Result<(String,)> {
        ic_cdk::call(self.0, "getMyPrincipal", ()).await
    }
    pub async fn getProjectCreatorAccountId(&self) -> Result<(AccountIdentifier__1,)> {
        ic_cdk::call(self.0, "getProjectCreatorAccountId", ()).await
    }
    pub async fn getRegistry(&self) -> Result<(Vec<(TokenIndex, AccountIdentifier__1)>,)> {
        ic_cdk::call(self.0, "getRegistry", ()).await
    }
    pub async fn getRewards(&self) -> Result<(Vec<String>,)> {
        ic_cdk::call(self.0, "getRewards", ()).await
    }
    pub async fn getTokens(&self) -> Result<(Vec<(TokenIndex, Metadata)>,)> {
        ic_cdk::call(self.0, "getTokens", ()).await
    }
    pub async fn http_request(&self, arg0: HttpRequest) -> Result<(HttpResponse,)> {
        ic_cdk::call(self.0, "http_request", (arg0,)).await
    }
    pub async fn http_request_streaming_callback(
        &self,
        arg0: HttpStreamingCallbackToken,
    ) -> Result<(HttpStreamingCallbackResponse,)> {
        ic_cdk::call(self.0, "http_request_streaming_callback", (arg0,)).await
    }
    pub async fn initRewards(&self, arg0: Vec<String>) -> Result<()> {
        ic_cdk::call(self.0, "initRewards", (arg0,)).await
    }
    pub async fn list(&self, arg0: ListRequest) -> Result<(Result_3,)> {
        ic_cdk::call(self.0, "list", (arg0,)).await
    }
    pub async fn list_bulk(
        &self,
        arg0: Vec<(TokenIndex, u64)>,
    ) -> Result<(Vec<(TokenIndex, u64)>,)> {
        ic_cdk::call(self.0, "list_bulk", (arg0,)).await
    }
    pub async fn listings(&self) -> Result<(Vec<(TokenIndex, Listing, Metadata)>,)> {
        ic_cdk::call(self.0, "listings", ()).await
    }
    pub async fn lock(
        &self,
        arg0: TokenIdentifier__1,
        arg1: u64,
        arg2: AccountIdentifier__1,
        arg3: SubAccount__1,
    ) -> Result<(Result_6,)> {
        ic_cdk::call(self.0, "lock", (arg0, arg1, arg2, arg3)).await
    }
    pub async fn metadata(&self, arg0: TokenIdentifier__1) -> Result<(Result_5,)> {
        ic_cdk::call(self.0, "metadata", (arg0,)).await
    }
    pub async fn mintManyNFTs(&self, arg0: Vec<Principal>) -> Result<(Vec<TokenIndex>,)> {
        ic_cdk::call(self.0, "mintManyNFTs", (arg0,)).await
    }
    pub async fn mintManyNFTsAID(
        &self,
        arg0: Vec<AccountIdentifier__1>,
    ) -> Result<(Vec<TokenIndex>,)> {
        ic_cdk::call(self.0, "mintManyNFTsAID", (arg0,)).await
    }
    pub async fn mintManyNFTsWithAsset(
        &self,
        arg0: Vec<Principal>,
        arg1: u32,
    ) -> Result<(Vec<TokenIndex>,)> {
        ic_cdk::call(self.0, "mintManyNFTsWithAsset", (arg0, arg1)).await
    }
    pub async fn payments(&self) -> Result<(Option<Vec<SubAccount__1>>,)> {
        ic_cdk::call(self.0, "payments", ()).await
    }
    pub async fn principalOwnsOne(&self, arg0: Principal) -> Result<(bool,)> {
        ic_cdk::call(self.0, "principalOwnsOne", (arg0,)).await
    }
    pub async fn removeRewards(&self) -> Result<()> {
        ic_cdk::call(self.0, "removeRewards", ()).await
    }
    pub async fn runHeartbeat(&self) -> Result<(bool,)> {
        ic_cdk::call(self.0, "runHeartbeat", ()).await
    }
    pub async fn setMinter(&self, arg0: Principal) -> Result<()> {
        ic_cdk::call(self.0, "setMinter", (arg0,)).await
    }
    pub async fn setProjectOwner(&self, arg0: Principal) -> Result<()> {
        ic_cdk::call(self.0, "setProjectOwner", (arg0,)).await
    }
    pub async fn setRewardRedeemed(
        &self,
        arg0: TokenIdentifier__1,
        arg1: candid::Nat,
        arg2: bool,
    ) -> Result<(Result_4,)> {
        ic_cdk::call(self.0, "setRewardRedeemed", (arg0, arg1, arg2)).await
    }
    pub async fn settle(&self, arg0: TokenIdentifier__1) -> Result<(Result_3,)> {
        ic_cdk::call(self.0, "settle", (arg0,)).await
    }
    pub async fn settlements(&self) -> Result<(Vec<(TokenIndex, AccountIdentifier__1, u64)>,)> {
        ic_cdk::call(self.0, "settlements", ()).await
    }
    pub async fn stats(
        &self,
    ) -> Result<(u64, u64, u64, u64, candid::Nat, candid::Nat, candid::Nat)> {
        ic_cdk::call(self.0, "stats", ()).await
    }
    pub async fn streamAsset(
        &self,
        arg0: candid::Nat,
        arg1: bool,
        arg2: serde_bytes::ByteBuf,
    ) -> Result<()> {
        ic_cdk::call(self.0, "streamAsset", (arg0, arg1, arg2)).await
    }
    pub async fn supply(&self, arg0: TokenIdentifier__1) -> Result<(Result_2,)> {
        ic_cdk::call(self.0, "supply", (arg0,)).await
    }
    pub async fn tokens(&self, arg0: AccountIdentifier__1) -> Result<(Result_1,)> {
        ic_cdk::call(self.0, "tokens", (arg0,)).await
    }
    pub async fn tokens_ext(&self, arg0: AccountIdentifier__1) -> Result<(MotokoResult,)> {
        ic_cdk::call(self.0, "tokens_ext", (arg0,)).await
    }
    pub async fn transactions(&self) -> Result<(Vec<Transaction>,)> {
        ic_cdk::call(self.0, "transactions", ()).await
    }
    pub async fn transfer(&self, arg0: TransferRequest) -> Result<(TransferResponse,)> {
        ic_cdk::call(self.0, "transfer", (arg0,)).await
    }
    pub async fn transfer_bulk(
        &self,
        arg0: Vec<(TokenIndex, AccountIdentifier__1)>,
    ) -> Result<(Vec<(TokenIndex, AccountIdentifier__1)>,)> {
        ic_cdk::call(self.0, "transfer_bulk", (arg0,)).await
    }
    pub async fn updateThumb(&self, arg0: String, arg1: File) -> Result<(Option<candid::Nat>,)> {
        ic_cdk::call(self.0, "updateThumb", (arg0, arg1)).await
    }
}

import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type Result = { 'Ok' : Uint32Array | number[] } |
  { 'Err' : string };
export type Result_1 = { 'Ok' : null } |
  { 'Err' : string };
export interface Wallets { 'nns_wallet' : Principal, 'nft_wallet' : Principal }
export interface _SERVICE {
  '__get_candid_interface_tmp_hack' : ActorMethod<[], string>,
  'add_to_whitelist' : ActorMethod<[Principal], Result>,
  'get_canister' : ActorMethod<[], string>,
  'get_snapshot' : ActorMethod<[], Array<[number, string]>>,
  'get_whitelist' : ActorMethod<[], Array<[number, Wallets]>>,
  'take_snapshot' : ActorMethod<[string], Result_1>,
}

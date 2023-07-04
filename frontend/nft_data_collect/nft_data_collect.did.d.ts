import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type Result = { 'Ok' : Uint32Array | number[] } |
  { 'Err' : string };
export type Result_1 = { 'Ok' : Array<[number, string]> } |
  { 'Err' : string };
export interface _SERVICE {
  '__get_candid_interface_tmp_hack' : ActorMethod<[], string>,
  'add_to_whitelist' : ActorMethod<[], Result>,
  'get_whitelist' : ActorMethod<[], Result_1>,
}

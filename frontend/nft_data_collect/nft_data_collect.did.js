export const idlFactory = ({ IDL }) => {
  const Result = IDL.Variant({ 'Ok' : IDL.Vec(IDL.Nat32), 'Err' : IDL.Text });
  const Wallets = IDL.Record({
    'nns_wallet' : IDL.Principal,
    'nft_wallet' : IDL.Principal,
  });
  const Result_1 = IDL.Variant({
    'Ok' : IDL.Vec(IDL.Tuple(IDL.Nat32, Wallets)),
    'Err' : IDL.Text,
  });
  return IDL.Service({
    '__get_candid_interface_tmp_hack' : IDL.Func([], [IDL.Text], ['query']),
    'add_to_whitelist' : IDL.Func([IDL.Principal], [Result], []),
    'get_whitelist' : IDL.Func([], [Result_1], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };

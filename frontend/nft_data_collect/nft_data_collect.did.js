export const idlFactory = ({ IDL }) => {
  const Result = IDL.Variant({ 'Ok' : IDL.Vec(IDL.Nat32), 'Err' : IDL.Text });
  const Result_1 = IDL.Variant({
    'Ok' : IDL.Vec(IDL.Tuple(IDL.Nat32, IDL.Text)),
    'Err' : IDL.Text,
  });
  return IDL.Service({
    '__get_candid_interface_tmp_hack' : IDL.Func([], [IDL.Text], ['query']),
    'add_to_whitelist' : IDL.Func([], [Result], []),
    'get_whitelist' : IDL.Func([], [Result_1], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };

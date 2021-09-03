/* eslint-disable @typescript-eslint/no-unused-vars */
export default ({ IDL }) => {
  const ProfileMetadata = IDL.Record({
    banner: IDL.Opt(IDL.Text),
    description: IDL.Opt(IDL.Text),
    emoji: IDL.Opt(IDL.Text),
    display_name: IDL.Opt(IDL.Text),
    version: IDL.Nat32,
    avatar: IDL.Opt(IDL.Text),
  });
  return IDL.Service({
    get_profile: IDL.Func(
      [IDL.Opt(IDL.Principal)],
      [IDL.Opt(ProfileMetadata)],
      [],
    ),
    name: IDL.Func([], [IDL.Text], ['query']),
    set_avatar: IDL.Func([IDL.Text], [], []),
    set_banner: IDL.Func([IDL.Text], [], []),
    set_description: IDL.Func([IDL.Text], [], []),
    set_display_name: IDL.Func([IDL.Text], [], []),
    set_emoji: IDL.Func([IDL.Text], [], []),
    set_profile: IDL.Func([ProfileMetadata], [], []),
  });
};
export const init = ({ IDL }) => [];

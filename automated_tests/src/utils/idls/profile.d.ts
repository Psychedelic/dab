import type { Principal } from '@dfinity/principal';

export interface ProfileMetadata {
  'banner' : [] | [string],
  'description' : [] | [string],
  'emoji' : [] | [string],
  'display_name' : [] | [string],
  'version' : number,
  'avatar' : [] | [string],
}
export default interface _SERVICE {
  'get_profile' : (arg_0: [] | [Principal]) => Promise<[] | [ProfileMetadata]>,
  'name' : () => Promise<string>,
  'set_avatar' : (arg_0: string) => Promise<undefined>,
  'set_banner' : (arg_0: string) => Promise<undefined>,
  'set_description' : (arg_0: string) => Promise<undefined>,
  'set_display_name' : (arg_0: string) => Promise<undefined>,
  'set_emoji' : (arg_0: string) => Promise<undefined>,
  'set_profile' : (arg_0: ProfileMetadata) => Promise<undefined>,
}

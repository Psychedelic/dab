import { Actor } from '@dfinity/agent';
import profileIDL from '../idls/profile.did';

import agent from '../agent';

const createActor = (canisterId: string) => {
  const actor = Actor.createActor(profileIDL, {
    canisterId,
    agent,
  });
  return actor;
};

export default createActor;

export const getProfileCanisterId = () => {
  if (process?.env?.PROFILE_CANISTER_ID) {
    return process.env.PROFILE_CANISTER_ID;
  }

  throw new Error('Oops! Missing environment variable PROFILE_CANISTER_ID!');
};

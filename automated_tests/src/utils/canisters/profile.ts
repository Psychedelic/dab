import { Actor } from '@dfinity/agent';
import agent from '../agent';
import { ProfileIDL, ProfileIDLFactory } from '../idls';

const createActor = (canisterId: string) => {
  const actor = Actor.createActor<ProfileIDL>(ProfileIDLFactory, {
    canisterId,
    agent,
  });
  return actor;
};

export default createActor;

export const getProfileCanisterId = 'REPLACE-THIS-WITH-PRINCIPAL-ID';

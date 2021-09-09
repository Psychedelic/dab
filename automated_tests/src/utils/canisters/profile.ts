import { Actor } from '@dfinity/agent';
// import system = require('system-commands');
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

export const getProfileCanisterId = 'ryjl3-tyaaa-aaaaa-aaaba-cai';

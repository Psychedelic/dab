import fetch from 'node-fetch';
import { HttpAgent } from '@dfinity/agent';
import { Ed25519KeyIdentity } from '@dfinity/identity';

const identity = Ed25519KeyIdentity.generate();
const fakeIdentity = Ed25519KeyIdentity.generate();

// @ts-ignore
const agent = new HttpAgent({ host: 'http://localhost:8000', fetch });

export const principal = identity.getPrincipal();
export const fakePrincipal = fakeIdentity.getPrincipal();

export default agent;

# 💠 The Token List

The Token Registry will work exactly like the NFT List. Any Token can get listed on this open registry, regardless of its standard (DIP20, EXT to start), adding metadata for UIs to surface (name, symbol, image, Canister ID, standard…)

Then UIs, apps, and DeFi experiences can consume this list & integrate it using DAB-js to integrate and auto-surface and support all tokens on the list for your users (showing their balance, allowing them to interact with them for example to make transfers), as well as anyone that’s added in the future, without having to do per-token or per-standard integrations.

- [Read our getting started guide](https://docs.dab.ooo/token-list/getting-started/)
- [DAB-js library - Repository](https://github.com/psychedelic/dab-js)
---

## 🧰 Interaction guide

As a developer that is looking to integrate DAB into an app/UI, your main point of interaction should be to use the [DAB-js library](https://github.com/psychedelic/dab-js), that also provides a standard wrapper to facilitate the integration to Tokens that use different standards. You can [read our documentation](https://docs.dab.ooo) to get detailed examples on how to do so.

**The mainnet canister ID for DAB's Token list is: `qwt65-nyaaa-aaaah-qcl4q-cai`**

----

## 0. ⚙️ Preparing your environment

To pull and install from [@Psychedelic](https://github.com/psychedelic) via the NPM CLI, you'll need:

- A Github account
- A Github personal access token (you can create a personal access token [here](https://github.com/settings/tokens))
- The personal access token with the correct scopes, **repo** and **read:packages** to be granted access to the [GitHub Package Registry](https://docs.github.com/en/packages/working-with-a-github-packages-registry/working-with-the-npm-registry#authenticating-to-github-packages).
- Authentication via `npm login`, using your Github email for the **username** and the **personal access token** as your **password**:

Once you have those ready, run:

```
npm login --registry=https://npm.pkg.github.com --scope=@Psychedelic
```

> **Note:** You only need to configure this once to install the package!
    On npm login provide your Github email as your username and the Personal access token as the password.
You can also setup your npm global settings to fetch from the Github registry everytime it finds a **@Psychdelic** package:

```sh
npm set //npm.pkg.github.com/:_authToken "$PAT"
```

⚠️ Alternatively, a token could be passed to the `.npmrc` as `//npm.pkg.github.com/:_authToken=xxxxxx` but we'd like to keep it clean, tokenless. Here's an example where the `PAT` is an environment variable:

```sh
@psychedelic:registry=https://npm.pkg.github.com
//npm.pkg.github.com/:_authToken=${PAT}
```

## 1. 🧰 Setting up DAB-js in your project

First, you need to install the DAB-js **npm package** into your project.

You can do so from the command line:
```js
npm install @psychedelic/dab-js@latest
```

Find more details about installing versions in the package page [here](https://github.com/Psychedelic/DAB-js/packages/987540)


---

## 2. 💠 Interacting with Tokens (getTokenActor, transfer, details)

To interact with the user's Tokens and, for example, trigger a transfer, you need to **initialize/get an Token actor object**. This is done using the **getTokenActor** method, where you need to pass:

- `canisterID`: the Canister ID of the token you want to interact with (e.g XTC)
- `agent`: and HttpAgent (instantiated with agent-js or Plug)
- `standard`: a str with the name of the Token standard (DIP20, EXT)
- `template`: an optional argument that enables access to non-standard methods.

**It's important to note:** without passing a template interface, you can use the following methods:

- send
- getMetadata
- getBalance
- burnXTC (in the case of XTC)

These are the common methods shared across all interfaces/standards that DAB-js wraps in its common universal interface. To **use other methods that are not shared across standards**, you can import and pass that standard's interface into the type variable of getTokenActor and use the base methods as "_method" (e.g. if transfer, do _transfer).

> (Current standards supported and string name: DIP20, EXT, XTC, WICP)
```js
import { getTokenActor } from '@psychedelic/dab-js'
export const getTokenActor = <T={}>({
  canisterId: string,
  agent: HttpAgent,
  standard: string
}): TokenActor => {
  return createTokenActor<T>(canisterID, agent, standard);
};
```

This should return an actor object with the following interfaces.

```js
export default class TokenActor {
  agent: HttpAgent;
  canisterId: string;
  constructor(canisterId: string, agent: HttpAgent) {
    this.agent = agent;
    this.canisterId = canisterId;
  }
  abstract send({to: string, from: string, amount: string}): Promise<SendResponse>;
  abstract getBalance(user: Principal): Promise<Balance>;
  abstract getMetadata(): Promise<Metadata>;
  abstract burnXTC({to: Principal, amount: string}): Promise<BurnResult>;
}
```

As you can see this actor contains the **standard javascript interface** of DAB's **Token standard wrapper**. It has generic calls to interact with tokens regardless of their standard (as long as their interface is wrapped in the standard wrapper).

- `send`: Request the transfer of a token balance the user owns to another address. 
- `getMetadata`: Returns the details of the token.
- `getBalance`: Returns the balance of a specific user.
- `burnXTC`: Request burning XTC to transfer raw unwrapped cycles to an address.

Here's a more detailed breakdown of the interfaces it returns:

```js
export type SendResponse =
  | { height: string }
  | { amount: string }
  | { transactionId: string };
export interface SendParams {
  to: string;
  from: string;
  amount: string;
}
export interface BurnParams {
  to: Principal;
  amount: string;
}
export interface Balance {
  value: string;
  decimals: number;
}
export type Metadata = FungibleMetadata | NonFungibleMetadata;
export interface FungibleMetadata {
  fungible: TokenMetaData & {
    metadata?: Int8Array[];
  };
}
export interface TokenMetaData {
  name: string;
  decimals: number;
  symbol: string;
}
export interface NonFungibleMetadata {
  nonfungible: {
    metadata: Int8Array[];
  };
```

### getBalance - Request a User's Balance for a Token

This method allows you to fetch an object with a `value`(str) `decimals`(number) with the balance a user owns in a specific token and its decimals.

Here, you would need to pass:

- `principal`: a str of the user's Principal ID you want to check for owned tokens.

> (See that in the variable TokenActor, we are instantiating the Token actor object, passing a canisterID for the token we want to interact with, an agent, and the name of the standard as a str).
```js
import { Principal } from '@dfinity/principal';
import { getTokenActor } from '@psychedelic/dab-js'
...
const getUserBalance = async () => {
  const principal = 'r4rmh-mbkzp-gv2na-yvly3-zcp3r-ocllf-pt3p3-zsri5-6gqvr-stvs2-4ae';
  const canisterId = 'utozz-siaaa-aaaam-qaaxq-cai';
  const standard = 'WICP';
  const TokenActor = getTokenActor({canisterId, agent, standard});
  const userTokens = await TokenActor.getBalance(Principal.fromText(principal));
}
getUserBalance();
```


### send - Request to Transfer a User's Token to a Different Address

This method allows you to request the transfer of an Token the passed identity owns in the token you have **initialized in the actor**.

In this method you need to pass:

- `to`: a str of a Principal ID for the destination address.
- `from`: origin address of the user whose balance is reduced.
- `amount`: the amount of tokens to be transferred.

> (See that in the variable TokenActor, we are instantiating the Token actor object, passing a canisterID for the token we want to interact with, an agent, and the name of the standard as a str).
```js
import { Principal } from '@dfinity/principal';
import { getTokenActor } from '@psychedelic/dab-js'
...
const send = async () => {
  const from = 'r4rmh-mbkzp-gv2na-yvly3-zcp3r-ocllf-pt3p3-zsri5-6gqvr-stvs2-4ae';
  const to = 'j63dc-rpowz-5pt7o-nah2l-z33hc-yacyk-ep3kc-fcxsp-evlxc-p3rv5-2qe'
  const canisterId = 'utozz-siaaa-aaaam-qaaxq-cai';
  const standard = 'WICP';
  const TokenActor = getTokenActor({canisterId, agent, standard});
  await TokenActor.transfer({from, to, amount: '1.2'});
}
send();
```

The transfer call, **if successful**  returns a send response. **If the transaction fails, it will return an error**.

### getMetadata - Fetch the Details of a Specific Token.

This method allows you to fetch an array with the details and metadata of token you **initialized in the actor**.

In this method, you don't need to pass anything.

> (See that in the variable TokenActor, we are instantiating the Token actor object, passing a canisterID for the token we want to interact with, an agent, and the name of the standard as a str).
```js
import { Principal } from '@dfinity/principal';
import { getTokenActor } from '@psychedelic/dab-js'
...
const getTokenDetails = async () => {
  const canisterId = 'utozz-siaaa-aaaam-qaaxq-cai';
  const standard = 'WICP';
  const TokenActor = getTokenActor({canisterId, agent, standard});
  const details = await TokenActor.getMetadata();
}
getTokenDetails()
```

This call returns one object with the metadata of the specific token queried.

---

### burnXTC - Burn XTC and Transfer Raw Cycles.

This method allows you to request burning (unwrapping) an amount of XTC into raw cycles to transfer them to a destination address:

In this method, you need to pass:

- `to`: a str of a Principal ID for the destination address.
- `amount`: the amount of tokens to be transferred.

> (See that in the variable TokenActor, we are instantiating the Token actor object, passing a canisterID for the token we want to interact with, an agent, and the name of the standard as a str).
```js
import { Principal } from '@dfinity/principal';
import { getTokenActor } from '@psychedelic/dab-js'
...
const burnXTC = async () => {
  const canisterId = 'utozz-siaaa-aaaam-qaaxq-cai';
  const to = 'r4rmh-mbkzp-gv2na-yvly3-zcp3r-ocllf-pt3p3-zsri5-6gqvr-stvs2-4ae';
  const standard = 'WICP';
  const TokenActor = getTokenActor({canisterId, agent, standard});
  const details = await TokenActor.burnXTC({to, amount: '1.2'});
}
burnXTC()
```

This call returns one object with the metadata of the specific token queried.

### NOTE:

The `details` field in this canister contains these information about the canister:
- **symbol** of the entry token canister -> `(String, DetailValue::Text(String))`
- **total_supply** of the entry token canister -> `(String, DetailValue::U64(u64))`
- **standard** of the entry token canister -> `(String, DetailValue::Text(String))`
- **verified** status of the entry token canister -> `(String, DetailValue::True) || (String, DetailValue::False)`
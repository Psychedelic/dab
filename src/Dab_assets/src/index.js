import { Actor, HttpAgent } from '@dfinity/agent';
import { idlFactory as Dab_idl, canisterId as Dab_id } from 'dfx-generated/Dab';

const agent = new HttpAgent();
const Dab = Actor.createActor(Dab_idl, { agent, canisterId: Dab_id });

document.getElementById("clickMeBtn").addEventListener("click", async () => {
  const name = document.getElementById("name").value.toString();
  const greeting = await Dab.greet(name);

  document.getElementById("greeting").innerText = greeting;
});

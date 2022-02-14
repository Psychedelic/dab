#!/usr/bin/env node

const fs = require("fs"),
      csv = require("csv-parser"),
      createCsvWriter = require('csv-writer').createObjectCsvWriter,
      program = require("inquirer"),
      { execSync } = require('child_process');

var results = [],
    inputFile = process.argv[2],
    ui = new program.ui.BottomBar();

const csvWriter = createCsvWriter({
    path: 'out.csv',
    header: [
        {id: 'name', title: 'name'},
        {id: 'id', title: 'principal_id'},
        {id: 'description', title: 'description'},
        {id: 'url', title: 'url'},
        {id: 'logo', title: 'logo_url'},
    ]
});
      

function main() {
    program
        .prompt([
            {
                type: 'list',
                name: 'functionality',
                message: 'What do you want to do?',
                choices: [
                    'Export the csv file of canister registry',
                    'Add / Update the NFT registry',
                    'Add / Update the tokens registry',
                    'Add / Update entries in the canister registry',
                    'List'
                ]
            }
        ])
        .then((answer) => {
            let stream = fs.createReadStream(inputFile)
                .pipe(csv());
            if (answer.functionality == 'Add / Update the NFT registry') {
                add_nft(stream);
            } else if (answer.functionality == 'Export the csv file of canister registry') {
                csv_canister_registry(stream);
            } else if (answer.functionality == 'List') {
                list(stream);
            } else if (answer.functionality == 'Add / Update entries in the canister registry') {
                add_canister(stream);
            } else if (answer.functionality == 'Add / Update the tokens registry') {
                add_tokens_entry(stream);
            }
        });
}

function list(stream) {
    let record;
    stream
        .on("data", (data) => {
            if (data.Name != '' && data.Description != '' && data.URL != '' && data.Logo != '' && data.ID != '') {
                let item = {
                    name: data.Name,
                    id: data.ID,
                    description: data.Description,
                    url: data.URL,
                    logo: data.Logo,
                };
                record += `(
                    "${data.ID}",
                    CanisterCategory::${data.Category}
                ),\n`;
            }
        })
        .on("end", () => {
            console.log(record);
        });
}

function csv_canister_registry(stream) {
    stream
        .on("data", (data) => {
            if (data.Name != '' && data.Description != '' && data.URL != '' && data.Logo != '' && data.ID != '') {
                let item = {
                    name: data.Name,
                    id: data.ID,
                    description: data.Description,
                    url: data.URL,
                    logo: data.Logo,
                };
                results.push(item);
            }
        })
        .on("end", () => {
            csvWriter
              .writeRecords(results)
              .then( () => console.log('The CSV file was written successfully'));
        });
}

function add_nft(stream) {
    stream
    .on("data", (data) => {
            if (data.name != '' && data.standard != '' && data.id != '' && data.description != '' && data.thumbnail != '') {
                let item = {
                    name: data.name,
                    id: data.id,
                    standard: data.standard,
                    description: data.description,
                    thumbnail: data.thumbnail,
                    frontend: data.frontend
                };
                results.push(item);
            } else {
              console.log(`Can't parse ${data.name}`);
            }
    })
    .on("end", () => {
        console.log(results);
        console.log(`Total amount of entries: ${results.length}`);
        program
            .prompt([
                {
                    type: 'confirm',
                    name: 'isThisOk',
                    message: 'Does this parsing look okay?',
                    default: false,
                },
                {
                    type: 'input',
                    name: 'address',
                    message: "Please enter canister's principal ID: "
                }
            ])
            .then((answers) => {
                if (answers.isThisOk) {
                    for (let i = 0; i < results.length; i++) {
                        let canister = results[i],
                            name = canister.name,
                            standard = canister.standard,
                            id = canister.id,
                            description = canister.description,
                            thumbnail = canister.thumbnail,
                            frontend = canister.frontend;
                        
                        if (frontend == '') {
                          const command = [
                            'dfx',
                            'canister',
                            '--network=ic',
                            '--no-wallet',
                            'call',
                            answers.address,
                            'add',
                            `"(record {principal_id= principal \\"${id}\\"; name= \\"${name}\\"; description= \\"${description}\\"; thumbnail= \\"${thumbnail}\\"; frontend= opt null; details= vec { record {\\"standard\\"; variant { Text= \\"${standard}\\" } } } })"`,
                        ];
                          try {
                            execSync(command.join(' '));
                        } catch (e) {
                            ui.log.write(`FAILED: ${name} because ${e}`);
                            continue;
                        }
                        ui.log.write(`ADDED: ${name}`);
                        ui.updateBottomBar(`${i + 1}/${results.length + 1}`);
                        } else {
                          const command = [
                            'dfx',
                            'canister',
                            '--network=ic',
                            '--no-wallet',
                            'call',
                            answers.address,
                            'add',
                            `"(record {principal_id= principal \\"${id}\\"; name= \\"${name}\\"; description= \\"${description}\\"; thumbnail= \\"${thumbnail}\\"; frontend= opt \\"${frontend}\\"; details= vec { record {\\"standard\\"; variant { Text= \\"${standard}\\" } } } })"`,
                        ];
                        try {
                          execSync(command.join(' '));
                      } catch (e) {
                          ui.log.write(`FAILED: ${name} because ${e}`);
                          continue;
                      }
                      ui.log.write(`ADDED: ${name}`);
                      ui.updateBottomBar(`${i + 1}/${results.length + 1}`);
                        }
                    }
                    ui.updateBottomBar('FINISHED');
                } 
            });
    });

}

function add_tokens_entry(stream) {
    stream
    .on("data", (data) => {
            if (data.Name != '' && data.Symbol != '' && data.Total_supply != '' && data.Standard != '' && data.Description != '' && data.ID != '' && data.Thumbnail != '') {
                let v = data.Verified == 'yes' ? 'True' : 'False';
                let item = {
                    name: data.Name,
                    id: data.ID,
                    standard: data.Standard,
                    description: data.Description,
                    thumbnail: data.Thumbnail,
                    frontend: data.Frontend,
                    total_supply: data.Total_supply,
                    symbol: data.Symbol,
                    verified: v
                };
                results.push(item);
            }
    })
    .on("end", () => {
        console.log(results);
        program
            .prompt([
                {
                    type: 'confirm',
                    name: 'isThisOk',
                    message: 'Does this parsing look okay?',
                    default: false,
                },
                {
                    type: 'input',
                    name: 'address',
                    message: "Please enter canister's principal ID: "
                }
            ])
            .then((answers) => {
                if (answers.isThisOk) {
                    for (let i = 0; i < results.length; i++) {
                        let canister = results[i],
                            name = canister.name,
                            standard = canister.standard,
                            id = canister.id,
                            description = canister.description,
                            thumbnail = canister.thumbnail,
                            frontend = canister.frontend,
                            total_supply = canister.total_supply,
                            symbol = canister.symbol,
                            verified = canister.verified;
                        
                        const command = [
                            'dfx',
                            'canister',
                            '--network=ic',
                            '--no-wallet',
                            'call',
                            answers.address,
                            'add',
                            `"(record {principal_id= principal \\"${id}\\"; name= \\"${name}\\"; description= \\"${description}\\"; thumbnail= \\"${thumbnail}\\"; frontend= opt \\"${frontend}\\"; details= vec { record {\\"standard\\"; variant { Text= \\"${standard}\\" } }; record {\\"symbol\\"; variant { Text= \\"${symbol}\\" } }; record {\\"total_supply\\"; variant { U64= ${total_supply} } }; record {\\"verified\\"; variant { ${verified} } } } })"`,
                        ];
                        try {
                            execSync(command.join(' '));
                        } catch (e) {
                            ui.log.write(`FAILED: ${name}`);
                            continue;
                        }
                        ui.log.write(`ADDED: ${name}`);
                        ui.updateBottomBar(`${i + 1}/${results.length + 1}`);
                    }
                    ui.updateBottomBar('FINISHED');
                } 
            });
    });

}

function add_canister(stream) {
    let dataStream = [
      {
        "id": "24pmb-qiaaa-aaaah-aannq-cai",
        "url": "https://2ji5m-raaaa-aaaah-aanoa-cai.raw.ic0.app/",
        "name": "Coinflip Backend",
        "description": "A coin-flip application example built to showcase integrations with the Plug wallet extension for the Internet Computer!",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/canister-logos/coin-flip.png",
        "category": "Games"
      },
      {
        "id": "qqfer-riaaa-aaaah-qcana-cai",
        "url": "https://ictuts.com/",
        "name": "ICTuts",
        "description": "10,000 randomly generated TuTs, a pharaoh-themed NFT collections on the Internet Computer.",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/logos/tuts.jpg",
        "category": "NFT"
      },
      {
        "id": "rdbii-uiaaa-aaaab-qadva-cai",
        "url": "https://rdbii-uiaaa-aaaab-qadva-cai.raw.ic0.app/",
        "name": "IC Canvas",
        "description": "IC Canvas is an interactive art project, running entirely on the Internet Computer. The experiment has completed - inspect the NFT's of the final state!",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/canister-logos/canvas.svg",
        "category": "Games"
      },
      {
        "id": "rrkah-fqaaa-aaaaa-aaaaq-cai",
        "url": "https://nns.ic0.app/",
        "name": "NNS Governance",
        "description": "The NNS controls all aspects of the Internet Computer network configuration and is responsible for performing many network management operations.",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/canister-logos/dfinity.png",
        "category": "Tools"
      },
      {
        "id": "j2nsf-iqaaa-aaaai-qanha-cai",
        "url": "https://j2nsf-iqaaa-aaaai-qanha-cai.ic0.app/",
        "name": "IC Chatbot",
        "description": "A support chatbot for Internet Computer.",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/canister-logos/dfinity.png",
        "category": "Service"
      },
      {
        "id": "73xld-saaaa-aaaah-qbjya-cai",
        "url": "https://twitter.com/olisav",
        "name": "Wing",
        "description": "An NFT photographic series created by the photographer @olisav ",
        "version": 0,
        "logo_url": "https://73xld-saaaa-aaaah-qbjya-cai.raw.ic0.app/?tokenid=tpx6i-sykor-uwiaa-aaaaa-b4ako-aaqca-aaaaz-a",
        "category": "NFT"
      },
      {
        "id": "stz5m-sqaaa-aaaah-qaggq-cai",
        "url": "https://stz5m-sqaaa-aaaah-qaggq-cai.raw.ic0.app/",
        "name": "Cycles Bet",
        "description": "Prvable secure and fair cycles bettign games.",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/canister-logos/cycles-bet.png",
        "category": "Games"
      },
      {
        "id": "utozz-siaaa-aaaam-qaaxq-cai",
        "url": "https://dank.ooo/wicp/",
        "name": "WICP",
        "description": "Wrapped version of the Internet Computer’s native ICP token that can be held by Principal IDs",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/logos/wicp-logo.png",
        "category": "Token"
      },
      {
        "id": "bxdf4-baaaa-aaaah-qaruq-cai",
        "url": "https://icpunks.com/",
        "name": "Wrapped ICPunks",
        "description": "IC Drip are randomly generated NFTs with meta-commerce shopping carts for outfits and personas stored on chain on the Internet Computer.",
        "version": 0,
        "logo_url": "https://qcg3w-tyaaa-aaaah-qakea-cai.raw.ic0.app/Token/1",
        "category": "NFT"
      },
      {
        "id": "r7nfb-aaaaa-aaaaj-qabla-cai",
        "url": "https://r7nfb-aaaaa-aaaaj-qabla-cai.raw.ic0.app/manage",
        "name": "Proof of humanity IC",
        "description": "A registry of only humans. One human one vote!",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/logos/pohic.jpg",
        "category": "Tools"
      },
      {
        "id": "qcg3w-tyaaa-aaaah-qakea-cai",
        "url": "https://icpunks.com/",
        "name": "ICPunks",
        "description": "10,000 randomly generated, unique collectible clowns with proof of ownership stored on the Internet Computer blockchain. Created as a reference to a meme comparing the Internet Computer token (ICP) with the Insane Clown Posse - an American hip hop duo founded in 1989.",
        "version": 0,
        "logo_url": "https://qcg3w-tyaaa-aaaah-qakea-cai.raw.ic0.app/Token/1",
        "category": "NFT"
      },
      {
        "id": "nntkg-vqaaa-aaaad-qamfa-cai",
        "url": "https://nntkg-vqaaa-aaaad-qamfa-cai.raw.ic0.app/",
        "name": "NFT Village",
        "description": "A friendly place for all of your favorite NFTs on the Internet Computer.",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/canister-logos/nftvillage.jpeg",
        "category": "NFT"
      },
      {
        "id": "xkbqi-2qaaa-aaaah-qbpqq-cai",
        "url": "https://k7gat-daaaa-aaaae-qaahq-cai.ic0.app/",
        "name": "ICPBunny",
        "description": "A bunny-themed NFT collection that uses utility tokens to enable yieldable and breedable NFTs on the Internet Computer.",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/ICPBunny.png",
        "category": "NFT"
      },
      {
        "id": "cnxby-3qaaa-aaaah-qcdpq-cai",
        "url": "http://andre-wee.com/",
        "name": "André Wee",
        "description": "André Wee, a newcomer to the NFT Art scene, is an experimental illustrator that jumps between both the virtual and physical world when he creates his craft.",
        "version": 0,
        "logo_url": "https://cnxby-3qaaa-aaaah-qcdpq-cai.raw.ic0.app/?cc=0&type=thumbnail&tokenid=b6qdx-lykor-uwiaa-aaaaa-b4aq3-4aqca-aaaaa-a",
        "category": "NFT"
      },
      {
        "id": "lhq4n-3yaaa-aaaai-qaniq-cai",
        "url": "http://nftstudio.biz/",
        "name": "Principia Mathematica",
        "description": "An Ode to Mathematics, a silent tribute to the greatest minds of all time.",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/principia.png",
        "category": "NFT"
      },
      {
        "id": "r7inp-6aaaa-aaaaa-aaabq-cai",
        "url": "https://github.com/dfinity/ic/tree/master/rs/nns/handlers/root",
        "name": "NNS Root",
        "description": "This is the handler for NNS canister upgrade proposals. ",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/canister-logos/dfinity.png",
        "category": "Tools"
      },
      {
        "id": "e3izy-jiaaa-aaaah-qacbq-cai",
        "url": "https://cronic.toniqlabs.com/",
        "name": "Cronic Critters",
        "description": "Cronics is a Play-to-earn NFT game being developed by ToniqLabs for the Internet Computer. Cronics  incorporates breeding mechanics, wearable NFTs and a p2e minigame ecosystem and more.",
        "version": 0,
        "logo_url": "https://e3izy-jiaaa-aaaah-qacbq-cai.raw.ic0.app/?tokenid=hancg-5ykor-uwiaa-aaaaa-b4aaq-maqca-aabuk-a",
        "category": "NFT"
      },
      {
        "id": "2ji5m-raaaa-aaaah-aanoa-cai",
        "url": "https://2ji5m-raaaa-aaaah-aanoa-cai.raw.ic0.app/",
        "name": "Coinflip Frontend",
        "description": "A coin-flip application example built to showcase integrations with the Plug wallet extension for the Internet Computer!",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/canister-logos/coin-flip.png",
        "category": "Games"
      },
      {
        "id": "crt3j-mqaaa-aaaah-qcdnq-cai",
        "url": "https://neilwhiteart.com/",
        "name": "Neil White",
        "description": "Neil White is a Miami-based contemporary artist. His work focuses on ironic portraits of historical icons and social commentary in the form of robots.",
        "version": 0,
        "logo_url": "https://crt3j-mqaaa-aaaah-qcdnq-cai.raw.ic0.app/?cc=0&type=thumbnail&tokenid=rtuc6-kqkor-uwiaa-aaaaa-b4aq3-maqca-aaaaa-q",
        "category": "NFT"
      },
      {
        "id": "grwwk-gaaaa-aaaah-aapma-cai",
        "url": "https://grwwk-gaaaa-aaaah-aapma-cai.raw.ic0.app/",
        "name": "ContentFly",
        "description": "Content Marketplace connecting high quality Creators with Brands, running on Dfinity Internet Computer",
        "version": 0,
        "logo_url": "https://grwwk-gaaaa-aaaah-aapma-cai.raw.ic0.app/apple-touch-icon.png",
        "category": "Social"
      },
      {
        "id": "n7ib3-4qaaa-aaaai-qagnq-cai",
        "url": "https://n7ib3-4qaaa-aaaai-qagnq-cai.raw.ic0.app/#/",
        "name": "ICApps",
        "description": "Apps powered by the Internet Computer.",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/canister-logos/icapps.svg",
        "category": "Service"
      },
      {
        "id": "kss7i-hqaaa-aaaah-qbvmq-cai",
        "url": "https://twitter.com/CeleberityI",
        "name": "ICelebrity",
        "description": "An NFT collection of 100 unique minted celebrities (Hollywood, Football Players, Singers and more).",
        "version": 0,
        "logo_url": "https://kss7i-hqaaa-aaaah-qbvmq-cai.raw.ic0.app/?tokenid=bubur-lykor-uwiaa-aaaaa-b4anl-eaqca-aaaae-a",
        "category": "NFT"
      },
      {
        "id": "sr4qi-vaaaa-aaaah-qcaaq-cai",
        "url": "https://interastrosc.com/",
        "name": "Internet Astronauts",
        "description": "Internet Astronauts is a collection of 10,000 unique digital astronaut NFT collectibles only found on the Internet Computer.",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/logos/internetastro.jpeg",
        "category": "NFT"
      },
      {
        "id": "m7sm4-2iaaa-aaaab-qabra-cai",
        "url": "https://m7sm4-2iaaa-aaaab-qabra-cai.raw.ic0.app/",
        "name": "Motoko Playground",
        "description": "A playground for the Internet Computer's native Motoko language, brought to you by DFINITY",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/canister-logos/motoko-playground.png",
        "category": "Tools"
      },
      {
        "id": "3pbcj-viaaa-aaaah-qaajq-cai",
        "url": "https://www.icpswap.com/",
        "name": "ICPSwap",
        "description": "Reinventing the Exchange as a Hub that Provides Full-stack Financial, Market, and DAO Services on the Internet Computer.",
        "version": 0,
        "logo_url": "https://3pbcj-viaaa-aaaah-qaajq-cai.raw.ic0.app/assets/favicon.ico",
        "category": "Service"
      },
      {
        "id": "rno2w-sqaaa-aaaaa-aaacq-cai",
        "url": "https://dfinity.org/",
        "name": "NNS Lifeline",
        "description": "Lifeline allows its upgrade and thus the fixing of all NNS canisters should some common (e.g. library or codegen) flaw be present in any of the NNS canisters.\r\n",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/canister-logos/dfinity.png",
        "category": "Tools"
      },
      {
        "id": "ryjl3-tyaaa-aaaaa-aaaba-cai",
        "url": "https://medium.com/dfinity/the-network-nervous-system-governing-the-internet-computer-1d176605d66a",
        "name": "NNS Ledger",
        "description": "The ledger canister implements a smart contract that holds accounts and balances and keeps a history of the transactions that affect accounts and balances.",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/canister-logos/dfinity.png",
        "category": "Token"
      },
      {
        "id": "owuqd-dyaaa-aaaah-qapxq-cai",
        "url": "https://twitter.com/myartbar/",
        "name": "ICPuzzle",
        "description": "The ICPuzzle NFT is an artistic NFT that is meant to invoke thought around individuality, community, and the beauty of the human condition. Each puzzle piece represents human individuality within humanity, a self-contained piece of a larger cohesive whole.",
        "version": 0,
        "logo_url": "https://owuqd-dyaaa-aaaah-qapxq-cai.raw.ic0.app/?tokenid=2e7o5-wykor-uwiaa-aaaaa-b4ad5-4aqca-aaagc-q",
        "category": "NFT"
      },
      {
        "id": "hdxhu-qqaaa-aaaai-aasnq-cai",
        "url": "https://hwqwz-ryaaa-aaaai-aasoa-cai.raw.ic0.app/",
        "name": "Candy Library",
        "description": "Candy Library NFTs - distribution licenses for a motoko software library, fund software!",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/dab/candy-lib.png",
        "category": "NFT"
      },
      {
        "id": "k4qsa-4aaaa-aaaah-qbvnq-cai",
        "url": "https://twitter.com/a3capas",
        "name": "The Universo",
        "description": "An NFT collection by a3capas. The Universo is where the chosen escape their mundane lives to astral travel and fuse their souls with the most powerful creatures.",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/dab/a3capas-logo.png",
        "category": "NFT"
      },
      {
        "id": "mx7fv-viaaa-aaaah-aarsa-cai",
        "url": "https://drip.land/",
        "name": "DRIP.LAND",
        "description": "Home of the Drip universe, an open source Internet Computer NFT inspired by Loot Project.",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/canister-logos/dripland.png",
        "category": "NFT"
      },
      {
        "id": "lc7ip-3iaaa-aaaah-aafva-cai",
        "url": "https://lc7ip-3iaaa-aaaah-aafva-cai.ic0.app/",
        "name": "Metaverse",
        "description": "Metaverse world game made during the first Internet Computer hackathon by Seb Thuillier",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/canister-logos/metaverse.png",
        "category": "Games"
      },
      {
        "id": "bxhqr-vyaaa-aaaah-aaqza-cai",
        "url": "https://cubic.place",
        "name": "Cubic",
        "description": "An experimental generative art project based on on-chain data on the Internet Computer.",
        "version": 0,
        "logo_url": "https://cubic.place/img/cubic.svg",
        "category": "NFT"
      },
      {
        "id": "lm5fh-ayaaa-aaaah-aafua-cai",
        "url": "https://lm5fh-ayaaa-aaaah-aafua-cai.ic0.app/",
        "name": "Texas Poker",
        "description": "Texas Holdem game on the Internet Computer.",
        "version": 0,
        "logo_url": "https://lm5fh-ayaaa-aaaah-aafua-cai.ic0.app/favicon.ico",
        "category": "Games"
      },
      {
        "id": "ckwhm-wiaaa-aaaah-qcdpa-cai",
        "url": "https://www.patternbased.com/",
        "name": "PatternBased",
        "description": "PatternBased is a boutique creative label at the intersection of art and technology.",
        "version": 0,
        "logo_url": "https://ckwhm-wiaaa-aaaah-qcdpa-cai.raw.ic0.app/?cc=0&type=thumbnail&tokenid=33iab-dikor-uwiaa-aaaaa-b4aq3-yaqca-aaaaa-q",
        "category": "NFT"
      },
      {
        "id": "ep54t-xiaaa-aaaah-qcdza-cai",
        "url": "https://icpuppies.io/",
        "name": "ICPuppies Wearables",
        "description": "Wearable collection for ICPuppies NFTs.",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/logos/emWjzvJf_400x400.jpg",
        "category": "NFT"
      },
      {
        "id": "ptodj-lqaaa-aaaah-qaeaq-cai",
        "url": "https://ptodj-lqaaa-aaaah-qaeaq-cai.raw.ic0.app/",
        "name": "Randomness Oracle",
        "description": "Randomness Oracle supplies the same system secure randomness to the caller, and at the same time also keeps a record of them.",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/canister-logos/dfinity.png",
        "category": "Tools"
      },
      {
        "id": "7e6iv-biaaa-aaaaf-aaada-cai",
        "url": "https://7e6iv-biaaa-aaaaf-aaada-cai.ic0.app/",
        "name": "Openchat",
        "description": "Chat. On the blockchain. In real time. Made possible by the Internet Computer. ",
        "version": 0,
        "logo_url": "https://7e6iv-biaaa-aaaaf-aaada-cai.ic0.app/apple-touch-icon.png",
        "category": "Social"
      },
      {
        "id": "c7fao-laaaa-aaaae-aaa4q-cai",
        "url": "https://distrikt.app/",
        "name": "Distrikt",
        "description": "A decentralized, professional social media network\r\nthat empowers users to own and control their identity and data.",
        "version": 0,
        "logo_url": "https://az5sd-cqaaa-aaaae-aaarq-cai.ic0.app/favicon.ico",
        "category": "Social"
      },
      {
        "id": "aaaaa-aa",
        "url": "https://sdk.dfinity.org/docs/interface-spec/index.html#ic-management-canister",
        "name": "IC Management",
        "description": "The Internet Computer provides functionality such as canister and user management. This functionality is exposed to external users and canisters via the IC management canister.\r",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/canister-logos/dfinity.png",
        "category": "Tools"
      },
      {
        "id": "5ymop-yyaaa-aaaah-qaa4q-cai",
        "url": "https://www.wtctoken.com/",
        "name": "WTC",
        "description": "WTC utilize the Extendable Token Standard for DFINITY's Internet Computer to wrap native computation cycles.\r\n",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/canister-logos/WTC.png",
        "category": "Token"
      },
      {
        "id": "vlhm2-4iaaa-aaaam-qaatq-cai",
        "url": "https://crowns.ooo",
        "name": "CAP Crowns",
        "description": "Crowns are a collection of 10,000 uniquely generated NFTs on the Internet Computer. With a mix of traditional and psychedelic materials, and a CAP-powered transaction history for full provenance.",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/logos/crowns-ooo.png",
        "category": "NFT"
      },
      {
        "id": "h5aet-waaaa-aaaab-qaamq-cai",
        "url": "https://dscvr.one/",
        "name": "DSCVR",
        "description": "A decentralized social news aggregator built on the Internet Computer.",
        "version": 0,
        "logo_url": "https://h5aet-waaaa-aaaab-qaamq-cai.raw.ic0.app/favicon.ico",
        "category": "Social"
      },
      {
        "id": "tde7l-3qaaa-aaaah-qansa-cai",
        "url": "https://cronic.toniqlabs.com/",
        "name": "Cronic Wearables",
        "description": "Wearable NFTs, usable with the Cronics NFT collection. A Play-to-earn NFT game being developed by ToniqLabs for the Internet Computer. Cronics  incorporates breeding mechanics, wearable NFTs, a p2e minigame ecosystem, and more.",
        "version": 0,
        "logo_url": "https://tde7l-3qaaa-aaaah-qansa-cai.raw.ic0.app/?tokenid=gvmdu-vikor-uwiaa-aaaaa-b4adm-qaqca-aakby-a",
        "category": "NFT"
      },
      {
        "id": "l2jyf-nqaaa-aaaah-qadha-cai",
        "url": "https://l2jyf-nqaaa-aaaah-qadha-cai.raw.ic0.app/",
        "name": "Saga Tarot",
        "description": "Daily Tarot card draws from Internet Computer blockchain.",
        "version": 0,
        "logo_url": "https://l2jyf-nqaaa-aaaah-qadha-cai.raw.ic0.app/favicon.svg",
        "category": "Games"
      },
      {
        "id": "3xwpq-ziaaa-aaaah-qcn4a-cai",
        "url": "https://app.sonic.ooo/",
        "name": "Sonic Swap",
        "description": " A supersonic suite of apps accelerating DeFi on Internet Computer to warp speed. Sonic Swap is an AMM where users can swap their tokens and earn fees by providing liquidity.",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/logos/Sonic-swap-logo.png",
        "category": "Service"
      },
      {
        "id": "cihkf-qyaaa-aaaah-qb7jq-cai",
        "url": "https://twitter.com/visions_gfx",
        "name": "ICMojis Items",
        "description": "Collection of items that will be integrated into the ICmojis Origins game in the future.",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/logos/items.png",
        "category": "NFT"
      },
      {
        "id": "ivg37-qiaaa-aaaab-aaaga-cai",
        "url": "https://ivg37-qiaaa-aaaab-aaaga-cai.ic0.app/#!/play",
        "name": "Reversi",
        "description": "Reversi game on the Internet Computer.",
        "version": 0,
        "logo_url": "https://ivg37-qiaaa-aaaab-aaaga-cai.raw.ic0.app/logo.png",
        "category": "Games"
      },
      {
        "id": "qoctq-giaaa-aaaaa-aaaea-cai",
        "url": "https://nns.ic0.app/",
        "name": "NNS Governance UI",
        "description": "Interface for the NNS Governance system for the Internet Computer.",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/canister-logos/dfinity.png",
        "category": "Tools"
      },
      {
        "id": "snpdi-6yaaa-aaaah-aauaq-cai",
        "url": "https://stg.fleek.ooo/",
        "name": "Fleek.ooo (Staging)",
        "description": "Fleek.ooo is a development platform for DFINITY's Internet Computer. Build sites, apps, or infrastructure.",
        "version": 0,
        "logo_url": "https://fleek.ooo/images/webclip.png",
        "category": "Service"
      },
      {
        "id": "rl4ub-oqaaa-aaaah-qbi3a-cai",
        "url": "https://t6ury-eiaaa-aaaaj-qabgq-cai.raw.ic0.app/",
        "name": "Metascore",
        "description": "Leaderboards and prizes for the DSCVR Season 2 Gaming Hackathon.",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/canister-logos/metascore.svg",
        "category": "Games"
      },
      {
        "id": "cdvmq-aaaaa-aaaah-qcdoq-cai",
        "url": "https://selaykarasu.com/",
        "name": "Selay Karasu",
        "description": "NFT collection by Selay Karasu, a multidisciplinary artist and creative director based in Istanbul.",
        "version": 0,
        "logo_url": "https://cdvmq-aaaaa-aaaah-qcdoq-cai.raw.ic0.app/?cc=0&type=thumbnail&tokenid=ta7sv-bqkor-uwiaa-aaaaa-b4aq3-uaqca-aaaaa-a",
        "category": "NFT"
      },
      {
        "id": "sygsn-caaaa-aaaaf-qaahq-cai",
        "url": "https://icme.io/",
        "name": "ICME",
        "description": "The Internet Computer's first no-code website builder.",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/canister-logos/icme-3.png",
        "category": "Service"
      },
      {
        "id": "3db6u-aiaaa-aaaah-qbjbq-cai",
        "url": "https://dvr6e-lqaaa-aaaai-qam5a-cai.raw.ic0.app/",
        "name": "Wrapped IC Drip",
        "description": "IC Drip are randomly generated NFTs with meta-commerce shopping carts for outfits and personas stored on chain on the Internet Computer.",
        "version": 0,
        "logo_url": "https://d3ttm-qaaaa-aaaai-qam4a-cai.raw.ic0.app/?tokenId=1",
        "category": "NFT"
      },
      {
        "id": "njgly-uaaaa-aaaah-qb6pa-cai",
        "url": "https://icpuppies.io/",
        "name": "ICPuppies",
        "description": "10,000 unique randomly generated 8-bit puppy NFTs on the Internet Computer.",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/logos/ICPuppies.png",
        "category": "NFT"
      },
      {
        "id": "btggw-4aaaa-aaaah-qcdgq-cai",
        "url": "https://twitter.com/ICPumpkin",
        "name": "IC Pumpkins",
        "description": "ICPumpkin is 2222 unique pumpkins! The collection consists of 20 really different cool characters, each with their own mood and costume.",
        "version": 0,
        "logo_url": "https://btggw-4aaaa-aaaah-qcdgq-cai.raw.ic0.app/?cc=0&type=thumbnail&tokenid=dbmzw-tykor-uwiaa-aaaaa-b4aqz-uaqca-aaaah-a",
        "category": "NFT"
      },
      {
        "id": "kqre2-2qaaa-aaaad-qamxa-cai",
        "url": "https://7xw5z-uqaaa-aaaad-qaqcq-cai.raw.ic0.app/",
        "name": "NFT Studio Frontend",
        "description": "NFT Studio allows users to Mint, Buy, Sell and Stake 3DWASM NFTs on the Internet Computer",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/canister-logos/nftstudio.png",
        "category": "NFT"
      },
      {
        "id": "zfjzz-4aaaa-aaaah-aasbq-cai",
        "url": "https://zfjzz-4aaaa-aaaah-aasbq-cai.raw.ic0.app/",
        "name": "The IC Gallery",
        "description": "A creative metaverse to gather all NFT enthusiasts and enjoy arts on the Internet Computer.",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/canister-logos/gallery.jpg",
        "category": "NFT"
      },
      {
        "id": "uzhxd-ziaaa-aaaah-qanaq-cai",
        "url": "https://twitter.com/icp_news",
        "name": "ICP News",
        "description": "An NFT collection set designed by the @icp_news artist with an Internet Computer theme.",
        "version": 0,
        "logo_url": "https://uzhxd-ziaaa-aaaah-qanaq-cai.raw.ic0.app/?tokenid=3qdzl-jikor-uwiaa-aaaaa-b4adi-eaqca-aaaad-q",
        "category": "NFT"
      },
      {
        "id": "rkp4c-7iaaa-aaaaa-aaaca-cai",
        "url": "https://sdk.dfinity.org/docs/token-holders/nns-app-quickstart.html",
        "name": "NNS Cycles Minting",
        "description": "The cycle’s minting canister burns tokens, mints new cycles, and sends these freshly minted cycles to the user’s canister.\r",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/canister-logos/dfinity.png",
        "category": "Tools"
      },
      {
        "id": "dknxi-2iaaa-aaaah-qceuq-cai",
        "url": "https://twitter.com/DBulls_NFT",
        "name": "Dfinity Bulls",
        "description": "8,888 Awesome Badass Bulls will ever be minted! P2E Game coming!",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/logos/bulls.PNG",
        "category": "NFT"
      },
      {
        "id": "he3ie-naaaa-aaaad-qbdlq-cai",
        "url": "https://app.sonic.ooo/",
        "name": "Sonic Frontend",
        "description": "Sonic's application frontend for its AMM platform, providing swaps, liquidity, and other DeFi protocols.",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/logos/Sonic-swap-logo.png",
        "category": "Service"
      },
      {
        "id": "b5el6-hqaaa-aaaah-qcdhq-cai",
        "url": "https://mobile.twitter.com/wildandwestnft",
        "name": "Wild & West",
        "description": "Wild and West: The Journey Begins.",
        "version": 0,
        "logo_url": "https://b5el6-hqaaa-aaaah-qcdhq-cai.raw.ic0.app/?cc=0&type=thumbnail&tokenid=vzzh5-uykor-uwiaa-aaaaa-b4aqz-4aqca-aaaa5-a",
        "category": "NFT"
      },
      {
        "id": "dslea-eiaaa-aaaae-aaa3a-cai",
        "url": "https://axon.ooo/",
        "name": "Axon",
        "description": "Multi-user multi-neuron management service on the Internet Computer.",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/canister-logos/axon-logo.svg",
        "category": "Tools"
      },
      {
        "id": "oeee4-qaaaa-aaaak-qaaeq-cai",
        "url": "https://dfinity.org/",
        "name": "Motoko Day Drop",
        "description": "Motoko-themed NFT Collection on the Internet Computer.",
        "version": 0,
        "logo_url": "https://oeee4-qaaaa-aaaak-qaaeq-cai.raw.ic0.app/?cc=0&type=thumbnail&tokenid=w5txv-gakor-uwiaa-aaaaa-cuaab-eaqca-aadhc-a",
        "category": "NFT"
      },
      {
        "id": "3qxje-uqaaa-aaaah-qcn4q-cai",
        "url": "https://explorer.cap.ooo/app-transactions/3qxje-uqaaa-aaaah-qcn4q-cai",
        "name": "Sonic Swap History",
        "description": "Sonic's CAP history instance for keeping record of transactions that occur in Sonic's Swap canister.",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/logos/Sonic-swap-logo.png",
        "category": "Tools"
      },
      {
        "id": "dvr6e-lqaaa-aaaai-qam5a-cai",
        "url": "https://dvr6e-lqaaa-aaaai-qam5a-cai.raw.ic0.app/",
        "name": "ICDrip Website",
        "description": "IC Drip are randomly generated meta-commerce shopping carts for outfits\r\n    and personas stored on chain. Stats, images, and other functionality are\r\n    intentionally omitted for others to interpret. Feel free to use IC Drip in\r\n    any way you want.",
        "version": 0,
        "logo_url": "https://dvr6e-lqaaa-aaaai-qam5a-cai.raw.ic0.app/favicon.ico",
        "category": "NFT"
      },
      {
        "id": "wxi2q-oiaaa-aaaaj-qab2q-cai",
        "url": "https://cake.aviate-labs.com/",
        "name": "Cake",
        "description": "Slices of cake celebrating the birthdays of IC builders. Proceeds fund open source development.",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/logos/cake-logo.png",
        "category": "NFT"
      },
      {
        "id": "ljyte-qiaaa-aaaah-qaiva-cai",
        "url": "https://ljyte-qiaaa-aaaah-qaiva-cai.raw.ic0.app/",
        "name": "Modclub",
        "description": "A decentralized moderation platform for the Internet Computer.",
        "version": 0,
        "logo_url": "https://ljyte-qiaaa-aaaah-qaiva-cai.raw.ic0.app/favicon.ico",
        "category": "Service"
      },
      {
        "id": "rdmx6-jaaaa-aaaaa-aaadq-cai",
        "url": "https://identity.ic0.app/",
        "name": "Internet Identity",
        "description": "Internet Identity is a blockchain authentication system enables you to sign in securely and anonymously to dapps on the Internet Computer.",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/canister-logos/dfinity.png",
        "category": "Service"
      },
      {
        "id": "aanaa-xaaaa-aaaah-aaeiq-cai",
        "url": "https://dank.ooo/xtc/",
        "name": "XTC",
        "description": "Cycles Token (XTC) is a token that allows users or developers to hold cycles with just a Principal ID, and send, trade, or develop canisters with them.",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/canister-logos/XTC.svg",
        "category": "Token"
      },
      {
        "id": "ja7sy-daaaa-aaaai-qaguq-cai",
        "url": "https://ja7sy-daaaa-aaaai-qaguq-cai.raw.ic0.app/",
        "name": "Portal",
        "description": "A Stream to Earn Platform Powered by the Internet Computer.",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/canister-logos/portal.jpg",
        "category": "Social"
      },
      {
        "id": "gyuaf-kqaaa-aaaah-qceka-cai",
        "url": "https://www.infernalvampires.com/",
        "name": "Infernal Vampire Colony",
        "description": "Infernal Vampire Colony is an initial collection of 666 Vampires, with another 6000 to be released in future.",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/logos/IVC_Logo%20(1).png",
        "category": "NFT"
      },
      {
        "id": "d3ttm-qaaaa-aaaai-qam4a-cai",
        "url": "https://dvr6e-lqaaa-aaaai-qam5a-cai.raw.ic0.app/",
        "name": "IC Drip",
        "description": "IC Drip are randomly generated NFTs with meta-commerce shopping carts for outfits and personas stored on chain on the Internet Computer.",
        "version": 0,
        "logo_url": "https://d3ttm-qaaaa-aaaai-qam4a-cai.raw.ic0.app/?tokenId=11",
        "category": "NFT"
      },
      {
        "id": "bid2t-gyaaa-aaaah-qcdea-cai",
        "url": "https://www.hauntedhamsters.io/",
        "name": "Haunted Hamsters",
        "description": "NFT set of 6666 Haunted Hamsters.",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/logos/HH.jpg",
        "category": "NFT"
      },
      {
        "id": "pnpu4-3aaaa-aaaah-qcceq-cai",
        "url": "https://chimps.icproject.org/",
        "name": "Infinite Chimps",
        "description": "Every Infinite Chimp in this NFT collection is a portrait of a rescued chimp that now lives peacefully at the sanctuary.",
        "version": 0,
        "logo_url": "https://pnpu4-3aaaa-aaaah-qcceq-cai.raw.ic0.app/?cc=0&type=thumbnail&tokenid=sqpc5-5akor-uwiaa-aaaaa-b4aqr-eaqca-aaaav-a",
        "category": "NFT"
      },
      {
        "id": "nbg4r-saaaa-aaaah-qap7a-cai",
        "url": "https://starverse.toniqlabs.com/",
        "name": "Starverse",
        "description": "Starverse is an NFT collection of rare and unique Stars, a collaboration between DSCVR and ToniqLabs. The Starverse symbolizes the unlimited potential of the Internet Computer with it’s infinite size and unstoppable nature.",
        "version": 0,
        "logo_url": "https://nbg4r-saaaa-aaaah-qap7a-cai.raw.ic0.app/?tokenid=wdyem-pikor-uwiaa-aaaaa-b4ad7-yaqca-aacsh-a",
        "category": "NFT"
      },
      {
        "id": "bzsui-sqaaa-aaaah-qce2a-cai",
        "url": "https://pokedstudio.com/",
        "name": "Poked Bots",
        "description": " 10,000 unique bot-themed collectible NFTs on the Internet Computer blockchain by Poked Studio.",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/logos/pokedbot.jpg",
        "category": "NFT"
      },
      {
        "id": "q6hjz-kyaaa-aaaah-qcama-cai",
        "url": "https://twitter.com/ICPBunny",
        "name": "Wrapped ICPBunny",
        "description": "Wrapped version of ICPBunny, a bunny-themed NFT collection that uses utility tokens to enable yieldable and breedable NFTs on the Internet Computer.",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/ICPBunny.png",
        "category": "NFT"
      },
      {
        "id": "rglue-kyaaa-aaaah-qakca-cai",
        "url": "https://rglue-kyaaa-aaaah-qakca-cai.ic0.app/",
        "name": "ICDrive",
        "description": "Secure and private decentralized storage app built on the Internet Computer.",
        "version": 0,
        "logo_url": "https://storageapi.fleek.co/fleek-team-bucket/canister-logos/icdrive.jpg",
        "category": "Service"
      },
      {
        "id": "i67uk-hiaaa-aaaae-qaaka-cai",
        "url": "https://i67uk-hiaaa-aaaae-qaaka-cai.raw.ic0.app/",
        "name": "Sudograph",
        "description": "Sudograph is a GraphQL database for the Internet Computer (IC).",
        "version": 0,
        "logo_url": "https://i67uk-hiaaa-aaaae-qaaka-cai.raw.ic0.app/favicon.svg",
        "category": "Tools"
      },
      {
        "id": "gevsk-tqaaa-aaaah-qaoca-cai",
        "url": "https://graci-aaaaa-aaaah-aaqjq-cai.raw.ic0.app/",
        "name": "ICMojis",
        "description": "A collection inspired in old school forum emotes, designed by the artist VisionsGFX, also part of the interactive strategy game ICMoji Origins.",
        "version": 0,
        "logo_url": "https://gevsk-tqaaa-aaaah-qaoca-cai.raw.ic0.app/?tokenid=vqwej-jikor-uwiaa-aaaaa-b4adq-qaqca-aaaac-q",
        "category": "NFT"
      }
    ];

    for (let i = 0; i < dataStream.length; i++) {
        let item = dataStream[i];

        const command = [
            'dfx',
            'canister',
            '--network=ic',
            '--no-wallet',
            'call',
            'curr3-vaaaa-aaaah-abbdq-cai',
            'add',
            `"(record {principal_id= principal \\"${item.id}\\"; name= \\"${item.name}\\"; description= \\"${item.description}\\"; thumbnail= \\"${item.logo_url}\\"; frontend= opt \\"${item.url}\\"; details= vec { record {\\"category\\"; variant { Text= \\"${item.category}\\" } } } })"`,
        ];
        try {
            execSync(command.join(' '));
        } catch (e) {
            console.log(`FAILED: ${item.name}`);
            continue;
        }
        console.log(`added ${item.name}`);
    }
    
    /*
    stream
    .on("data", (data) => {
            if (data.Name != '' && data.Description != '' && data.URL != '' && data.Logo != '' && data.ID != '') {
                let item = {
                    name: data.Name,
                    id: data.ID,
                    description: data.Description,
                    url: data.URL,
                    logo: data.Logo,
                    category: data.Category
                };
                results.push(item);
            }
    })
    .on("end", () => {
        console.log(results);
        program
            .prompt([
                {
                    type: 'confirm',
                    name: 'isThisOk',
                    message: 'Does this parsing look okay?',
                    default: false,
                },
                {
                    type: 'input',
                    name: 'address',
                    message: "Please enter canister's principal ID: "
                }
            ])
            .then((answers) => {
                if (answers.isThisOk) {
                    for (let i = 0; i < results.length; i++) {
                        let canister = results[i],
                            name = canister.name,
                            description = canister.description,
                            id = canister.id,
                            url = canister.url,
                            logo = canister.logo,
                            category = canister.category;
                        
                        const command = [
                            'dfx',
                            'canister',
                            '--network=ic',
                            '--no-wallet',
                            'call',
                            answers.address,
                            'add_canister',
                            `"(principal \\"${id}\\", record {name= \\"${name}\\"; url= \\"${url}\\"; description= \\"${description}\\"; category= variant {${category}}; logo_url= \\"${logo}\\"})"`,
                        ];
                        try {
                            execSync(command.join(' '));
                        } catch (e) {
                            ui.log.write(`FAILED: ${name}`);
                            continue;
                        }
                        ui.log.write(`ADDED: ${name}`);
                        ui.updateBottomBar(`${i + 1}/${results.length + 1}`);
                    }
                    ui.updateBottomBar('FINISHED');
                }
            });
    }); */
}

main();
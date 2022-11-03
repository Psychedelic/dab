#!/usr/bin/env node
const fs = require("fs"),
  csv = require("csv-parser"),
  createCsvWriter = require('csv-writer').createObjectCsvWriter,
  program = require("inquirer"),
  { execSync } = require('child_process');

var canister_ids = require('../canister_ids.json');

var registries = {
  "nft": canister_ids['nft']['ic'],
  "token": canister_ids['tokens']['ic'],
  "canister_registry": canister_ids['registry']['ic'],
};

var results = [],
  inputFile = process.argv[2],
  ui = new program.ui.BottomBar();

function main() {
  program
    .prompt([
      {
        type: 'list',
        name: 'action',
        message: 'What do you want to do?',
        choices: [
          'Update the NFT registry',
          'Update the token registry',
          'Update the canister registry',
          'Export a csv file from the NFT registry',
          'Export a csv file from the token registry',
          'Export a csv file from the canister registry',
        ]
      }
    ])
    .then((answer) => {
      let stream = fs.createReadStream(inputFile)
        .pipe(csv());
      if (answer.action == 'Update the NFT registry') {
        update_nft(stream);
      } else if (answer.action == 'Update the token registry') {
        update_token(stream);
      } else if (answer.action == 'Update the canister registry') {
        update_canister(stream);
      } else if (answer.action == 'Export a csv file from the NFT registry') {
        nft_csv(stream);
      } else if (answer.action == 'Export a csv file from the token registry') {
        token_csv(stream);
      } else if (answer.action == 'Export a csv file from the canister registry') {
        canister_csv(stream);
      }
    });
}

function update_nft(stream) {
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
            name: 'parsingConfirmation',
            message: 'Does this parsing look okay?',
            default: false,
          }
        ])
        .then((answers) => {
          if (!answers.parsingConfirmation) {
            console.log("Operation aborted.")
            return;  
        }
        for (let i = 0; i < results.length; i++) {
          let entry = results[i];
          if (entry.frontend == '') {
            const command = [
              'dfx',
              'canister',
              '--network=ic',
              'call',
              registries.nft,
              'add',
              `"(null, record {principal_id= principal \\"${entry.id}\\"; name= \\"${entry.name}\\"; description= \\"${entry.description}\\"; thumbnail= \\"${entry.thumbnail}\\"; frontend= null; details= vec { record {\\"standard\\"; variant { Text= \\"${entry.standard}\\" } } } })"`,
            ];
            try {
              execSync(command.join(' '));
            } catch (e) {
              ui.log.write(`FAILED: ${entry.name} because ${e}`);
              continue;
            }
          } else {
            const command = [
              'dfx',
              'canister',
              '--network=ic',
              'call',
              registries.nft,
              'add',
              `"(null, record {principal_id= principal \\"${entry.id}\\"; name= \\"${entry.name}\\"; description= \\"${entry.description}\\"; thumbnail= \\"${entry.thumbnail}\\"; frontend= opt \\"${entry.frontend}\\"; details= vec { record {\\"standard\\"; variant { Text= \\"${entry.standard}\\" } } } })"`,
            ];
            try {
              execSync(command.join(' '));
            } catch (e) {
              ui.log.write(`FAILED: ${entry.name} because ${e}`);
              continue;
            }
          }
          ui.log.write(`ADDED: ${entry.name}`);
          ui.updateBottomBar(`${i + 1}/${results.length + 1}`);
        }
        ui.updateBottomBar('FINISHED');
    });
  });
}

function update_token(stream) {
  stream
    .on("data", (data) => {
      if (data.Name != '' && data.Fee != '' && data.Decimals != '' && data.Symbol != '' && data.Total_supply != '' && data.Standard != '' && data.Description != '' && data.ID != '' && data.Thumbnail != '') {
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
          fee: data.Fee,
          decimals: data.Decimals,
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
            name: 'parsingConfirmation',
            message: 'Does this parsing look okay?',
            default: false,
          }
        ])
        .then((answers) => {
          if (!answers.parsingConfirmation) {
            console.log("Operation aborted.");
            return;
          }
            for (let i = 0; i < results.length; i++) {
              let entry = results[i];

              const command = [
                'dfx',
                'canister',
                '--network=ic',
                'call',
                registries.token,
                'add',
                `'(null, record {principal_id= principal "${entry.id}"; name= "${entry.name}"; description= "${entry.description}"; thumbnail= "${entry.thumbnail}"; frontend= opt "${entry.frontend}"; details= vec { record {"symbol"; variant { Text= "${entry.symbol}" } }; record {"standard"; variant { Text= "${entry.standard}" } }; record {"total_supply"; variant { U64= ${entry.total_supply} } }; record {"verified"; variant { ${entry.verified} } }; record {"decimals"; variant { U64=${entry.decimals} } }; record {"fee"; variant { U64=${entry.fee} } } } })'`,
              ];
              try {
                execSync(command.join(' '));
              } catch (e) {
                ui.log.write(`FAILED: ${entry.name}`);
                continue;
              }
              ui.log.write(`ADDED: ${entry.name}`);
              ui.updateBottomBar(`${i + 1}/${results.length + 1}`);
            }
            ui.updateBottomBar('FINISHED');
        });
    });
}

function update_canister(stream) {
  stream
    .on("data", (data) => {
      if (data.Category != '' && data.Name != '' && data.Description != '' && data.Thumbnail != '' && data.ID != '') {
        let item = {
          name: data.Name,
          id: data.ID,
          description: data.Description,
          frontend: data.Frontend,
          thumbnail: data.Thumbnail,
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
            name: 'parsingConfirmation',
            message: 'Does this parsing look okay?',
            default: false,
          }
        ])
        .then((answers) => {
          if (!answers.parsingConfirmation) {
            console.log("Operation aborted.");
          }
            for (let i = 0; i < results.length; i++) {
              let entry = results[i];

              if (thumbnail != '') {
                const command = [
                  'dfx',
                  'canister',
                  '--network=ic',
                  'call',
                  registries.canister_registry,
                  'add',
                  `"(null, record {principal_id= principal \\"${entry.id}\\"; name= \\"${entry.name}\\"; description= \\"${entry.description}\\"; thumbnail= \\"${entry.thumbnail}\\"; frontend= opt \\"${entry.frontend}\\"; details= vec { record {\\"category\\"; variant { Text= \\"${entry.category}\\" } } } })"`,
                ];
                try {
                  execSync(command.join(' '));
                } catch (e) {
                  ui.log.write(`FAILED: ${entry.name}`);
                  continue;
                }
              } else {
                const command = [
                  'dfx',
                  'canister',
                  '--network=ic',
                  'call',
                  registries.canister_registry,
                  'add',
                  `"(null, record {principal_id= principal \\"${entry.id}\\"; name= \\"${entry.name}\\"; description= \\"${entry.description}\\"; thumbnail= \\"${entry.thumbnail}\\"; frontend= null; details= vec { record {\\"category\\"; variant { Text= \\"${entry.category}\\" } } } })"`,
                ];
                try {
                  execSync(command.join(' '));
                } catch (e) {
                  ui.log.write(`FAILED: ${entry.name}`);
                  continue;
                }
              }
              ui.log.write(`ADDED: ${entry.name}`);
              ui.updateBottomBar(`${i + 1}/${results.length + 1}`);
            }
            ui.updateBottomBar('FINISHED');
        });
    });
}

const csvWriter = createCsvWriter({
  path: 'out.csv',
  header: [
    { id: 'name', title: 'name' },
    { id: 'principal_id', title: 'principal_id' },
    { id: 'description', title: 'description' },
    { id: 'frontend', title: 'frontend' },
    { id: 'thumbnail', title: 'thumbnail' },
    { id: 'submitter', title: 'submitter' },
    { id: 'last_updated_by', title: 'last_updated_by' },
    { id: 'last_updated_at', title: 'last_updated_at' },
    { id: 'details', title: 'details' },
  ]
});


function canister_csv(stream) {
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
        .then(() => console.log('The CSV file was written successfully'));
    });
}

main();
#!/usr/bin/env node

const fs = require("fs"),
  csv = require("csv-parser"),
  createCsvWriter = require('csv-writer').createObjectCsvWriter,
  program = require("inquirer"),
  { execSync } = require('child_process');

var results = [],
  inputFile = process.argv[2],
  ui = new program.ui.BottomBar();

function main() {
  program
    .prompt([
      {
        type: 'list',
        name: 'functionality',
        message: 'What do you want to do?',
        choices: [
          'Add / Update the NFT registry',
          'Add / Update the tokens registry',
          'Add / Update the canister registry',
          'Export a csv file from NFT registry',
          'Export a csv file from tokens registry',
          'Export a csv file from canister registry',
        ]
      }
    ])
    .then((answer) => {
      let stream = fs.createReadStream(inputFile)
        .pipe(csv());
      if (answer.functionality == 'Add / Update the NFT registry') {
        add_nft(stream);
      } else if (answer.functionality == 'Add / Update the canister registry') {
        add_canister(stream);
      } else if (answer.functionality == 'Add / Update the tokens registry') {
        add_token(stream);
      } else if (answer.functionality == 'Export a csv file from canister registry') {
        canister_csv(stream);
      }
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
                  `"(record {principal_id= principal \\"${id}\\"; name= \\"${name}\\"; description= \\"${description}\\"; thumbnail= \\"${thumbnail}\\"; frontend= null; details= vec { record {\\"standard\\"; variant { Text= \\"${standard}\\" } } } })"`,
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

function add_token(stream) {
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
                frontend = canister.frontend,
                thumbnail = canister.thumbnail,
                category = canister.category;

              if (thumbnail != '') {
                const command = [
                  'dfx',
                  'canister',
                  '--network=ic',
                  '--no-wallet',
                  'call',
                  answers.address,
                  'add',
                  `"(record {principal_id= principal \\"${id}\\"; name= \\"${name}\\"; description= \\"${description}\\"; thumbnail= \\"${thumbnail}\\"; frontend= opt \\"${frontend}\\"; details= vec { record {\\"category\\"; variant { Text= \\"${category}\\" } } } })"`,
                ];
                try {
                  execSync(command.join(' '));
                } catch (e) {
                  ui.log.write(`FAILED: ${name}`);
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
                  `"(record {principal_id= principal \\"${id}\\"; name= \\"${name}\\"; description= \\"${description}\\"; thumbnail= \\"${thumbnail}\\"; frontend= null; details= vec { record {\\"category\\"; variant { Text= \\"${category}\\" } } } })"`,
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
            }
            ui.updateBottomBar('FINISHED');
          }
        });
    });
}

const csvWriter = createCsvWriter({
  path: 'out.csv',
  header: [
    { id: 'name', title: 'name' },
    { id: 'id', title: 'principal_id' },
    { id: 'description', title: 'description' },
    { id: 'url', title: 'url' },
    { id: 'logo', title: 'logo_url' },
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
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
                    'Add / Update NFT registry',
                    'Add / Update entries in the canister registry',
                    'List'
                ]
            }
        ])
        .then((answer) => {
            let stream = fs.createReadStream(inputFile)
                .pipe(csv());

            if (answer.functionality == 'Add / Update NFT canister') {
                add_nft(stream);
            } else if (answer.functionality == 'Export the csv file of canister registry') {
                csv_canister_registry(stream);
            } else if (answer.functionality == 'List') {
                list(stream);
            } else {
                add_canister(stream);
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
            if (data.name != '' && data.standard != '' && data.id != '' && data.description != '' && data.icon != '') {
                let item = {
                    name: data.name,
                    id: data.id,
                    standard: data.standard,
                    description: data.description,
                    icon: data.icon,
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
                            icon = canister.icon;
                        
                        const command = [
                            'dfx',
                            'canister',
                            '--network=ic',
                            '--no-wallet',
                            'call',
                            answers.address,
                            'add',
                            `"(record {principal_id= principal \\"${id}\\"; name= \\"${name}\\"; standard= \\"${standard}\\"; description= \\"${description}\\"; icon= \\"${icon}\\"})"`,
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
    });
}

main();
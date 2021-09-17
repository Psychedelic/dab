#!/usr/bin/env node

const fs = require("fs"),
      csv = require("csv-parser"),
      program = require("inquirer"),
      { execSync } = require('child_process');

var results = [];

var inputFile = process.argv[2];

let stream = fs.createReadStream(inputFile)
                .pipe(csv());

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
                            'call',
                            answers.address,
                            'add',
                            `"(record {principal_id= principal \\"${id}\\"; name= \\"${name}\\"; standard= \\"${standard}\\"; description= \\"${description}\\"; icon= \\"${icon}\\"})"`,
                        ];
                        execSync(command.join(' '));
                        console.log(`Add: ${name}`);
                    }
                }
            });
    });

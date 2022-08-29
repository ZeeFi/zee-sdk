import log from 'loglevel';
import { program } from 'commander';

import * as zee from '@zeefi/oracle-sdk';

program.version('0.0.1');
log.setLevel(log.levels.INFO);

export function programCommand(name: string) {
  return program
    .command(name)
    .option(
      '-c, --config <path>',
      'path to your aptos config.yml (generated with "aptos init")'
    )
    .option('-p, --profile <PROFILE>', 'aptos config profile to use', 'default')
    .option('-l, --log-level <string>', 'log level', setLogLevel);
}

function setLogLevel(value: any, _prev: any) {
  if (value == null) {
    return;
  }
  log.info('setting the log value to: ' + value);
  log.setLevel(value);
}

function errorColor(str: string) {
  // Add ANSI escape codes to display text in red.
  return `\x1b[31m${str}\x1b[0m`;
}

export const CONFIG_PATH =
  '/Users/valekar/Projects/zee/zee-sdk/packages/cli/.aptos/config.yaml';

/********************* Add Feed  command **********************/

programCommand('tokens:add-feed')
  .description('')
  .argument('<price>')
  .argument('<decimals>')
  .action(async (price: string, decimals: string, options) => {
    let { profile, config } = options;
    //console.log(config);
    //let configPath = zee.utils.readConfig(config);

    let configPath = zee.utils.readConfig(CONFIG_PATH);

    await zee.api.addFeedV1({
      price: +price,
      decimals: +decimals,
      aptosAccount: configPath.account,
      clusterUrl: zee.config.DEVNET_NODE_URL,
    });
  });
/********************* Add Feed  command **********************/

/********************* Get Feed  command **********************/

programCommand('tokens:get-feed')
  .description('')
  .action(async (options) => {
    let { profile, config } = options;
    //console.log(config);
    let configPath = zee.utils.readConfig(config);

    let resources = await config.client.getAccountResources(
      configPath.account.address()
    );

    console.log(
      resources.find(
        (r) =>
          r.type == config.account.address().toString() + '::tokens::Aggregator'
      ).data
    );
  });
/********************* Get Feed  command **********************/

/********************* Initialize command **********************/

programCommand('tokens:initialize')
  .description('')
  .argument('<id>')
  .argument('<name>')
  .argument('<symbol>')
  .action(async (id, name, symbol, options) => {
    let { profile, config } = options;

    //console.log(config);
    let configPath = zee.utils.readConfig(config);

    await zee.api.initializeTokensOracleV1({
      clusterUrl: zee.config.DEVNET_NODE_URL,
      aptosAccount: configPath.account,
      version: +id,
      oracleName: name,
      oracleSymbol: symbol,
    });
  });

/********************* Initialize command **********************/

program
  .configureOutput({
    // Visibly override write routines as example!
    writeOut: (str) => process.stdout.write(`[OUT] ${str}`),
    writeErr: (str) => process.stdout.write(`[ERR] ${str}`),
    // Highlight errors in color.
    outputError: (str, write) => write(errorColor(str)),
  })
  .parse(process.argv);

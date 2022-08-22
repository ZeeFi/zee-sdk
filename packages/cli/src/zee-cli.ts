import log from "loglevel";
import { program } from "commander";

import * as zee from "@zee/oracle";

program.version("0.0.1");
log.setLevel(log.levels.INFO);

export function programCommand(name: string) {
  return program
    .command(name)
    .requiredOption(
      "-c, --config <path>",
      'path to your aptos config.yml (generated with "aptos init")'
    )
    .option("-p, --profile <PROFILE>", "aptos config profile to use", "default")
    .option("-l, --log-level <string>", "log level", setLogLevel);
}

function setLogLevel(value: any, _prev: any) {
  if (value == null) {
    return;
  }
  log.info("setting the log value to: " + value);
  log.setLevel(value);
}

function errorColor(str: string) {
  // Add ANSI escape codes to display text in red.
  return `\x1b[31m${str}\x1b[0m`;
}

export const CONFIG_PATH = "../../oracle/ap_oracle/.aptos/config.yaml";

/********************* Add Feed  command **********************/

program
  .command("tokens:add-feed")
  .description("")
  .argument("<price>")
  .argument("<decimals>")
  .action(async (price: string, decimals: string) => {
    let config = zee.utils.readConfig(CONFIG_PATH);

    await zee.api.addFeedV1({
      price: +price,
      decimals: +decimals,
      aptosAccount: config.account,
      clusterUrl: zee.config.DEVNET_NODE_URL,
    });
  });
/********************* Add Feed  command **********************/

/********************* Get Feed  command **********************/

program
  .command("tokens:get-feed")
  .description("")
  .action(async () => {
    let config = zee.utils.readConfig(CONFIG_PATH);

    let resources = await config.client.getAccountResources(
      config.account.address()
    );

    console.log(
      resources.find(
        (r) =>
          r.type == config.account.address().toString() + "::tokens::Aggregator"
      ).data
    );
  });
/********************* Get Feed  command **********************/

/********************* Initialize command **********************/

program
  .command("tokens:initialize")
  .description("")
  .argument("<id>")
  .argument("<name>")
  .action(async (id: string, name: string) => {
    let config = zee.utils.readConfig(CONFIG_PATH);

    await zee.api.initializeTokensOracleV1({
      clusterUrl: zee.config.DEVNET_NODE_URL,
      aptosAccount: config.account,
      version: +id,
      oracleName: name,
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

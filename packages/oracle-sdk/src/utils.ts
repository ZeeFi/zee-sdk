import {
  AptosAccount,
  HexString,
  MaybeHexString,
  TxnBuilderTypes,
  Types,
} from 'aptos';
import * as fs from 'fs';
import * as yaml from 'yaml';
import { AptosClient } from 'aptos';
import { NODE_URL } from './config';

export const readConfig = (
  configPath: string = '.aptos/config.yaml',
  profile: string = 'default'
) => {
  const ymlContent = fs.readFileSync(configPath, {
    encoding: 'utf-8',
  });
  const result = yaml.parse(ymlContent);

  const url = result.profiles[profile].rest_url;
  const privateKeyStr = result.profiles[profile].private_key;

  const privateKey = new HexString(privateKeyStr);

  if (!url) {
    throw new Error(`Expect rest_url to be present in ${profile} profile`);
  }
  if (!privateKeyStr) {
    throw new Error(`Expect private_key to be present in ${profile} profile`);
  }

  const client = new AptosClient(result.profiles[profile].rest_url);
  const account = new AptosAccount(privateKey.toUint8Array());
  console.log(`Using address ${account.address().hex()}`);
  return { client, account };
};
// aptos client
export const aptosClient = new AptosClient(NODE_URL);

// send transaction
export const sendTransaction = async (
  client: AptosClient,
  wallet: AptosAccount,
  scriptFunctionPayload: TxnBuilderTypes.TransactionPayloadEntryFunction
) => {
  // Create a raw transaction out of the transaction payload
  const rawTxn = await client.generateRawTransaction(
    wallet.address(),
    scriptFunctionPayload,
    { maxGasAmount: 1000n, gastUnitPrice: 2n }
  );

  // Sign the raw transaction with private key
  const bcsTxn = AptosClient.generateBCSTransaction(wallet, rawTxn);
  // Submit the transaction
  const transactionRes = await client.submitSignedBCSTransaction(bcsTxn);

  await client.waitForTransaction(transactionRes.hash);
  const txDetails = (await client.getTransactionByHash(
    transactionRes.hash
  )) as Types.UserTransaction;
  console.log(txDetails);
};

export const getAptosAccount = (wallet: MaybeHexString) => {
  const privateKey: HexString = HexString.ensure(wallet);
  return new AptosAccount(privateKey.toUint8Array());
};

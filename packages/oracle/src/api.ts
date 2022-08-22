import { MaybeHexString } from 'aptos';

import { AptosClient } from 'aptos';
import { getAptosAccount, sendTransaction } from './utils';
import {
  buildAddFeedScriptFunction,
  buildGetFeedScriptFunction,
  buildInitializeScriptFunction,
} from './script-functions';
import {
  AddFeedApiArgs,
  GetFeedApiArgs,
  InitializeTokenOracleApiArgs,
} from './types';

// todo add init token oracle
const initializeTokensOracleV1 = async (args: InitializeTokenOracleApiArgs) => {
  try {
    let sender: MaybeHexString = args.wallet.account;
    const aptosClient = new AptosClient(args.clusterUrl);

    const initializeTokensOracleScript = buildInitializeScriptFunction({
      version: args.version,
      oracleName: args.oracleName,
    });

    await sendTransaction(
      aptosClient,
      getAptosAccount(sender),
      initializeTokensOracleScript
    );
  } catch (err) {
    throw err;
  }
};

const addFeedV1 = async (args: AddFeedApiArgs) => {
  try {
    let sender: MaybeHexString = args.wallet.account;
    const aptosClient = new AptosClient(args.clusterUrl);

    const addFeedScript = buildAddFeedScriptFunction({
      price: args.price,
      decimals: args.decimals,
      lastUpdate: new Date().toDateString(),
    });

    await sendTransaction(aptosClient, getAptosAccount(sender), addFeedScript);
  } catch (err) {
    throw err;
  }
};

const getFeed = async (args: GetFeedApiArgs) => {
  try {
    let sender: MaybeHexString = args.wallet.account;
    const aptosClient = new AptosClient(args.clusterUrl);

    const getFeedScript = buildGetFeedScriptFunction({});

    await sendTransaction(aptosClient, getAptosAccount(sender), getFeedScript);
  } catch (err) {}
};

export { initializeTokensOracleV1, addFeedV1, getFeed };

import { AptosAccount } from 'aptos';

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
import { MODULE_NAME } from './config';

// todo add init token oracle
const initializeTokensOracleV1 = async (args: InitializeTokenOracleApiArgs) => {
  try {
    let sender: AptosAccount | null = null;
    if (args.aptosAccount) {
      sender = args.aptosAccount;
    } else {
      sender = getAptosAccount(args.wallet.account);
    }

    if (sender == null) {
      throw new Error('Wallet not provided');
    }

    const aptosClient = new AptosClient(args.clusterUrl);

    const initializeTokensOracleScript = buildInitializeScriptFunction({
      version: args.version,
      oracleName: args.oracleName,
      moduleName: MODULE_NAME,
    });

    await sendTransaction(aptosClient, sender, initializeTokensOracleScript);
  } catch (err) {
    throw err;
  }
};

const addFeedV1 = async (args: AddFeedApiArgs) => {
  try {
    let sender: AptosAccount | null = null;
    if (args.aptosAccount) {
      sender = args.aptosAccount;
    } else {
      sender = getAptosAccount(args.wallet.account);
    }
    if (sender == null) {
      throw new Error('Wallet not provided');
    }
    const aptosClient = new AptosClient(args.clusterUrl);

    const addFeedScript = buildAddFeedScriptFunction({
      price: args.price,
      decimals: args.decimals,
      lastUpdate: Date.now().toString(),
      moduleName: MODULE_NAME,
    });

    await sendTransaction(aptosClient, sender, addFeedScript);
  } catch (err) {
    throw err;
  }
};

const getFeed = async (args: GetFeedApiArgs) => {
  try {
    let sender: AptosAccount | null = null;
    if (args.aptosAccount) {
      sender = args.aptosAccount;
    } else {
      sender = getAptosAccount(args.wallet.account);
    }
    if (sender == null) {
      throw new Error('Wallet not provided');
    }
    const aptosClient = new AptosClient(args.clusterUrl);

    const getFeedScript = buildGetFeedScriptFunction({
      moduleName: MODULE_NAME,
    });

    await sendTransaction(aptosClient, sender, getFeedScript);
  } catch (err) {}
};

//const getAccFeeds = async (args : )

export { initializeTokensOracleV1, addFeedV1, getFeed };

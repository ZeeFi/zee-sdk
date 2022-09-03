import { AptosAccount } from 'aptos';

import { AptosClient } from 'aptos';
import { getAptosAccount, sendTransaction } from './utils';
import {
  buildAddFeedScriptFunction,
  buildInitializeTokenScriptFunction,
  buildInitializeAggregatorScriptFunction,
  buildExampleLogTokenFeedScriptFunction,
} from './script-functions';
import {
  AddFeedApiArgs,
  InitializeAggregatorOracleApiArgs,
  InitializeTokenOracleApiArgs,
  LogTokenFeedApiArgs,
} from './types';

// init aggregator oracle
const initializeAggregatorOracleV1 = async (
  args: InitializeAggregatorOracleApiArgs
) => {
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

    const initializeTokensOracleScript =
      buildInitializeAggregatorScriptFunction({
        version: args.version,
        aggregatorName: args.aggregratorName,
        moduleName: args.moduleName,
      });

    await sendTransaction(aptosClient, sender, initializeTokensOracleScript);
  } catch (err) {
    throw err;
  }
};

const initializeTokenOracleV1 = async (args: InitializeTokenOracleApiArgs) => {
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

    const initializeTokensOracleScript = buildInitializeTokenScriptFunction({
      tokenName: args.tokenName,
      tokenSymbol: args.tokenSymbol,
      moduleName: args.moduleName,
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
      tokenSymbol: args.tokenSymbol,
      price: args.price,
      decimals: args.decimals,
      lastUpdate: Date.now().toString(),
      moduleName: args.moduleName,
    });

    await sendTransaction(aptosClient, sender, addFeedScript);
  } catch (err) {
    throw err;
  }
};

const exampleLogTokenFeedV1 = async (args: LogTokenFeedApiArgs) => {
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

    const addFeedScript = buildExampleLogTokenFeedScriptFunction({
      tokenSymbol: args.tokenSymbol,
      moduleName: args.moduleName,
    });

    await sendTransaction(aptosClient, sender, addFeedScript);
  } catch (err) {
    throw err;
  }
};

// const getFeed = async (args: GetFeedApiArgs) => {
//   try {
//     let sender: AptosAccount | null = null;
//     if (args.aptosAccount) {
//       sender = args.aptosAccount;
//     } else {
//       sender = getAptosAccount(args.wallet.account);
//     }
//     if (sender == null) {
//       throw new Error('Wallet not provided');
//     }
//     const aptosClient = new AptosClient(args.clusterUrl);

//     const getFeedScript = buildGetFeedScriptFunction({
//       moduleName: MODULE_NAME,
//     });

//     await sendTransaction(aptosClient, sender, getFeedScript);
//   } catch (err) {}
// };

export {
  initializeAggregatorOracleV1,
  initializeTokenOracleV1,
  addFeedV1,
  exampleLogTokenFeedV1,
};

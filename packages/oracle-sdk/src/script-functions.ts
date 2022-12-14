import { BCS, TxnBuilderTypes } from 'aptos';

const buildInitializeAggregatorScriptFunction = (args: {
  version: number;
  aggregatorName: string;
  moduleName: string;
}) => {
  return new TxnBuilderTypes.TransactionPayloadEntryFunction(
    TxnBuilderTypes.EntryFunction.natural(
      // Fully qualified module name, `AccountAddress::ModuleName`
      args.moduleName,
      // Module function
      'initialize_aggregator',
      [],
      [
        BCS.bcsSerializeU8(args.version),
        BCS.bcsSerializeStr(args.aggregatorName),
      ]
    )
  );
};

const buildInitializeTokenScriptFunction = (args: {
  tokenName: string;
  tokenSymbol: string;
  moduleName: string;
}) => {
  return new TxnBuilderTypes.TransactionPayloadEntryFunction(
    TxnBuilderTypes.EntryFunction.natural(
      // Fully qualified module name, `AccountAddress::ModuleName`
      args.moduleName,
      // Module function
      'initialize_token',
      [],
      [
        BCS.bcsSerializeStr(args.tokenName),
        BCS.bcsSerializeStr(args.tokenSymbol),
      ]
    )
  );
};

// const buildGetFeedScriptFunction = (args: { moduleName: string }) => {
//   return new TxnBuilderTypes.TransactionPayloadEntryFunction(
//     TxnBuilderTypes.EntryFunction.natural(
//       // Fully qualified module name, `AccountAddress::ModuleName`
//       args.moduleName,
//       // Module function
//       'get_feed',
//       [],
//       []
//     )
//   );
// };

const buildAddFeedScriptFunction = (args: {
  tokenSymbol: string;
  price: number;
  decimals: number;
  lastUpdate: string;
  moduleName: string;
}) => {
  return new TxnBuilderTypes.TransactionPayloadEntryFunction(
    TxnBuilderTypes.EntryFunction.natural(
      // Fully qualified module name, `AccountAddress::ModuleName`
      args.moduleName,
      // Module function
      'add_feed',
      [],
      [
        BCS.bcsSerializeStr(args.tokenSymbol),
        BCS.bcsSerializeU128(args.price),
        BCS.bcsSerializeU8(args.decimals),
        BCS.bcsSerializeStr(args.lastUpdate),
      ]
    )
  );
};

const buildExampleLogTokenFeedScriptFunction = (args: {
  tokenSymbol: string;
  moduleName: string;
}) => {
  return new TxnBuilderTypes.TransactionPayloadEntryFunction(
    TxnBuilderTypes.EntryFunction.natural(
      // Fully qualified module name, `AccountAddress::ModuleName`
      args.moduleName,
      // '0xb42175fd5ce66ec91aa52942199f63766f1a90e62fb3bd5f455413fb8f4a75a5::oracle',
      // Module function
      'log_token_feed',
      [],
      [BCS.bcsSerializeStr(args.tokenSymbol)]
    )
  );
};

export {
  buildInitializeAggregatorScriptFunction,
  buildInitializeTokenScriptFunction,
  buildAddFeedScriptFunction,
  buildExampleLogTokenFeedScriptFunction,
};

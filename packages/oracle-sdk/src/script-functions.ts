import { BCS, TxnBuilderTypes } from 'aptos';

const buildInitializeScriptFunction = (args: {
  version: number;
  oracleName: string;
  moduleName: string;
}) => {
  return new TxnBuilderTypes.TransactionPayloadEntryFunction(
    TxnBuilderTypes.EntryFunction.natural(
      // Fully qualified module name, `AccountAddress::ModuleName`
      args.moduleName,
      // Module function
      'initialize',
      [],
      [BCS.bcsSerializeU8(args.version), BCS.bcsSerializeStr(args.oracleName)]
    )
  );
};

const buildGetFeedScriptFunction = (args: { moduleName: string }) => {
  return new TxnBuilderTypes.TransactionPayloadEntryFunction(
    TxnBuilderTypes.EntryFunction.natural(
      // Fully qualified module name, `AccountAddress::ModuleName`
      args.moduleName,
      // Module function
      'get_feed',
      [],
      []
    )
  );
};

const buildAddFeedScriptFunction = (args: {
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
        BCS.bcsSerializeU128(args.price),
        BCS.bcsSerializeU8(args.decimals),
        BCS.bcsSerializeStr(args.lastUpdate),
      ]
    )
  );
};

export {
  buildInitializeScriptFunction,
  buildGetFeedScriptFunction,
  buildAddFeedScriptFunction,
};

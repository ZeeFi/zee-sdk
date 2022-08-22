import { BCS, TxnBuilderTypes } from 'aptos';

const buildInitializeScriptFunction = (args: {
  version: number;
  oracleName: string;
}) => {
  return new TxnBuilderTypes.TransactionPayloadEntryFunction(
    TxnBuilderTypes.EntryFunction.natural(
      // Fully qualified module name, `AccountAddress::ModuleName`
      '0xce938e214d7b44a98a9acf23ecc1b507e453c143d1026a935834271df6f5f07e::tokens',
      // Module function
      'initialize',
      [],
      [BCS.bcsSerializeU8(args.version), BCS.bcsSerializeStr(args.oracleName)]
    )
  );
};

const buildGetFeedScriptFunction = ({}) => {
  return new TxnBuilderTypes.TransactionPayloadEntryFunction(
    TxnBuilderTypes.EntryFunction.natural(
      // Fully qualified module name, `AccountAddress::ModuleName`
      '0xce938e214d7b44a98a9acf23ecc1b507e453c143d1026a935834271df6f5f07e::tokens',
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
}) => {
  return new TxnBuilderTypes.TransactionPayloadEntryFunction(
    TxnBuilderTypes.EntryFunction.natural(
      // Fully qualified module name, `AccountAddress::ModuleName`
      '0xce938e214d7b44a98a9acf23ecc1b507e453c143d1026a935834271df6f5f07e::tokens',
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

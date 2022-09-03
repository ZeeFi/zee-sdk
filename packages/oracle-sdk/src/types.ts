import { AptosAccount } from 'aptos';

interface BaseApiArgs {
  wallet?: any;
  aptosAccount?: AptosAccount;
  clusterUrl: string;
}

export interface InitializeAggregatorOracleApiArgs extends BaseApiArgs {
  version: number;
  aggregratorName: string;
}

export interface InitializeTokenOracleApiArgs extends BaseApiArgs {
  tokenName: string;
  tokenSymbol: string;
}

export interface AddFeedApiArgs extends BaseApiArgs {
  price: number;
  decimals: number;
  tokenSymbol: string;
}

export interface GetFeedApiArgs extends BaseApiArgs {}

export enum WalletType {
  MartianWallet,
  FewchaWallet,
  None,
}

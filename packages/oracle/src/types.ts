interface BaseApiArgs {
  wallet: any;
  clusterUrl: string;
}

export interface InitializeTokenOracleApiArgs extends BaseApiArgs {
  version: number;
  oracleName: string;
}

export interface AddFeedApiArgs extends BaseApiArgs {
  price: number;
  decimals: number;
}

export interface GetFeedApiArgs extends BaseApiArgs {}

export enum WalletType {
  MartianWallet,
  FewchaWallet,
  None,
}

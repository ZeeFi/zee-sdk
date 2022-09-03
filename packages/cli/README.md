# Oracle commands 


## Initialize Aggregator

```bash 
ts-node cli/src/zee-cli.ts tokens:initialize-aggregator <id> <name> --config "cli/.aptos/config.yaml" --module-name "0xda9724a10e323b710f08087951e88b9b2818196f0159a111beb5a394d969416c::tokens"
```

## Initialize Token

```bash 
ts-node cli/src/zee-cli.ts tokens:initialize-token <name> <symbol> --config "cli/.aptos/config.yaml" --module-name "0xda9724a10e323b710f08087951e88b9b2818196f0159a111beb5a394d969416c::tokens"
```


## Get Tokens
```bash
ts-node cli/src/zee-cli.ts tokens:get-feed --config "cli/.aptos/config.yaml"  --module-name "0xda9724a10e323b710f08087951e88b9b2818196f0159a111beb5a394d969416c::tokens"
```



## Example Log token feed 
```bash 
ts-node cli/src/zee-cli.ts tokens:example-log-token-feed  "ETH"  --config "cli/.aptos/config.yaml"  --module-name "0xb42175fd5ce66ec91aa52942199f63766f1a90e62fb3bd5f455413fb8f4a75a5::oracle"
```
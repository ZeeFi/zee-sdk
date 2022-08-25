# Oracle commands 


## Add Tokens

```bash 
ts-node cli/src/zee-cli.ts tokens:add-feed <price> <decimals> --config "cli/.aptos/config.yaml"
```


## Get Tokens
```bash
ts-node cli/src/zee-cli.ts tokens:get-feed --config "cli/.aptos/config.yaml"
```


## Initialize Tokens

```bash
ts-node cli/src/zee-cli.ts tokens:initialize <id> <name> <symbol> --config "cli/.aptos/config.yaml"

```
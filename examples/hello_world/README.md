# Hello World

## Build

```bash
rm -rf ./layer
mkdir -p ./layer/extensions

cargo build --release
cp ../../target/release/aws-lambda-sidecar-manager ./layer/extensions/

sam build
```

## Local Test

```bash
sam local invoke
```

## Deploy

```bash
sam deploy
```

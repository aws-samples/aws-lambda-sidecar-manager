# AWS Lambda Web Gateway

This example shows how this can work with [AWS Lambda Web Gateway](https://github.com/aws-samples/aws-lambda-web-gateway).

## Build

```bash
rm -rf ./layer
mkdir -p ./layer/extensions

[ ! -f aws-lambda-web-gateway ] && git clone https://github.com/aws-samples/aws-lambda-web-gateway.git && cd aws-lambda-web-gateway && cargo build --release && cd ..
cp ./aws-lambda-web-gateway/target/release/lambda-web-gateway ./layer/

cargo build --release
cp ../../target/release/aws-lambda-sidecar-manager ./layer/extensions/

sam build
```

## Deploy

```bash
sam deploy --stack-name SidecarManagerExampleWebGateway --resolve-s3 --capabilities CAPABILITY_IAM
```

## Remote Test

```bash
sam remote invoke CallerFunction --stack-name SidecarManagerExampleWebGateway
```

## Clean Up

```bash
sam delete --stack-name SidecarManagerExampleWebGateway --no-prompts
```

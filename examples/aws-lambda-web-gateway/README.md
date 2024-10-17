# AWS Lambda Web Gateway

This example shows how this can work with [AWS Lambda Web Gateway](https://github.com/aws-samples/aws-lambda-web-gateway) to call other AWS Lambda functions from an AWS Lambda function.

> [!IMPORTANT]
> Calling other AWS Lambda functions from an AWS Lambda function is an **_anti pattern_** and is not recommended in production. This is just an example to show how this can work.

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

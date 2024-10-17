# AWS Lambda Sidecar Manager

[![version](https://img.shields.io/github/v/tag/aws-samples/aws-lambda-sidecar-manager?label=release&style=flat-square)](https://github.com/aws-samples/aws-lambda-sidecar-manager/releases/latest)
![license](https://img.shields.io/github/license/aws-samples/aws-lambda-sidecar-manager?style=flat-square)

An AWS Lambda layer to run any sidecar process in the same sandbox as the handler function.

## Usage

### As a Lambda Layer

1. Build the binary by running `cargo build --release`.
2. Create a zip, copy `target/release/aws-lambda-sidecar-manager` into the zip at `extensions/aws-lambda-sidecar-manager`.
3. Upload the zip as an AWS Lambda layer. Add the layer to your AWS Lambda function.
4. Add a file `aws-lambda-sidecar.yaml` in your function code with the [configuration](#configuration) below.

### Configuration

```yaml
targets:
  - command: echo 123 # any shell command
    shell: bash # optional. default: sh
```

### Environment Variables

- `AWS_LAMBDA_SIDECAR_MANAGER_CONFIG`
  - The path to the configuration file.
  - Default: `${LAMBDA_TASK_ROOT}/aws-lambda-sidecar.yaml`, which is in your function code directory.

## [CHANGELOG](./CHANGELOG.md)
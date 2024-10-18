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

readiness: # optional
  # poll until a URL returns a status code
  - http:
      url: http://localhost:8080/ready
      method: GET # optional. default: GET
      status: 200 # optional. default: 200
    interval: 10 # in ms, optional. default: 1
  # poll until a command returns 0
  - exec:
      command: echo ready
      shell: bash # optional. default: sh
      status: 0 # optional. default: 0
    interval: 10
```

### Environment Variables

- `AWS_LAMBDA_SIDECAR_MANAGER_CONFIG`
  - The path to the configuration file.
  - Default: `/var/task/aws-lambda-sidecar.yaml`, which is usually in your function code directory.

## [CHANGELOG](./CHANGELOG.md)

## Security

See [CONTRIBUTING](CONTRIBUTING.md#security-issue-notifications) for more information.

## License

This library is licensed under the MIT-0 License. See the LICENSE file.

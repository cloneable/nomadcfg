# nomadcfg

A CLI tool for defining HashiCorp Nomad job specs in jsonnet. Unofficial. Not
related to HashiCorp.

Very experimental! The code is still PoC quality. Don't use yet.

## About

The official Nomad CLI tool `nomad` is able to ingest job specifications in JSON
instead of HCL, which allows other configuration languages to produce such job
specs. `nomadcfg` is an attempt to use jsonnet for this. It's similar to
`kubecfg` for Kubernetes.

## Installation

```shell
cargo install --locked nomadcfg
```

```shell
cargo install --git https://github.com/cloneable/nomadcfg
```

## Usage

myjob.jsonnet:

```jsonnet
local job(name) = {
  Type: 'service',
  ID: '%s-%s' % [self.Namespace, self.Name],
  Name: name,
  Namespace: 'biz',
};

{
  Job: job('myjob') {
    // job definition
  }

  // helper definitions (ignored by nomadcfg)
}
```

```shell
nomadcfg --spec myjob.jsonnet
```

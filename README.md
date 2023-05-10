# Workflow-RS

## Introduction

Workflow-RS is an orchestrator engine written in Rust. It is designed to be lightweight, easy to use, and extensible. It
is inspired by the Airflow project in Python.

## Building

```bash
cargo build --release
```

The binary will be located in `target/release/workflow-rs`.

## Running

```bash
cd ./target/release/
./workflow-rs
```

## Contribute

### Install comitizen and pre-commit:

Using `brew`,

```
brew install commitizen pre-commit
```

Using `pip`,

```
brew install commitizen pre-commit
```

Then run,

```
pre-commit install --hook-type commit-msg
pre-commit install --hook-type pre-push
```
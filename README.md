# AOS-ZMKL Worker

## Overview

**AOS-ZMKL Worker** is a Rust-based project that integrates zero-knowledge machine learning (ZKML) to handle cryptographic proof generation and verification for machine learning models. This worker is designed to work as part of a larger decentralized system, utilizing zero-knowledge proofs to ensure privacy and integrity in model inference and verification.

The project is based on `ezkl`, a framework for integrating zero-knowledge proofs with machine learning models, providing a secure and verifiable method for proving model outputs without revealing sensitive data.

## Features

- **Zero-Knowledge Proofs**: Generates zk proofs for machine learning models, ensuring both privacy and integrity in decentralized environments.
- **Integration with ezkl**: Uses the ezkl library for model compilation, proof generation, and verification.
- **Supports ONNX Models**: Works with ONNX-based machine learning models to generate cryptographic proofs.

## Requirements

- **Rust**: Ensure you have Rust installed. You can install Rust by following the [official guide](https://www.rust-lang.org/tools/install).
- **ONNX Model**: The worker operates on models in ONNX format, a standard format for machine learning models.
- **ezkl**: This worker relies on the `ezkl` library for zero-knowledge proof generation and verification.

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/hetu-project/aos-zmkl-worker.git
   cd aos-zmkl-worker
   ```

2. Initialize the submodules:

   ```bash
   git submodule update --init --recursive
   ```

3. Build the project using Rust:

   ```bash
   cargo build --release
   ```

## Usage

### Running the Server

The worker can also be run as an HTTP server, providing API endpoints for proof generation and verification. To run the server:

```bash
cargo run --release
```

The server exposes the following API endpoints:

- **`/api1/v1/prove`**: Generates a zk proof for the provided input.
- **`/api1/v1/verify`**: Verifies a zk proof.
- **`/api1/v1/ping`**: Simple ping endpoint for health checks.
- **`/api1/v1/healthcheck`**: Returns the health status of the worker.

## Configuration

- **ONNX Model Path**: The default ONNX model path is `./models/network.onnx`. You can change this by modifying the environment variable `ONNX_MODEL_PATH`.
- **Proof Settings**: Proof-related settings can be modified through the generated `settings.json` file during setup.

## Contribution

Contributions to the project are welcome. Please fork the repository, create a feature branch, and submit a pull request.

---

Let me know if you want to include additional details or make adjustments.

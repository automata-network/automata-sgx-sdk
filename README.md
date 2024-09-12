# Automata SGX SDK
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)

This repository contains the source code for the Automata SGX SDK, which is a software development kit for building secure enclaves on the Intel SGX platform. The SDK provides a set of APIs and tools to help developers build secure applications that run within an SGX enclave.

The Automata SGX SDK is built on top of [https://github.com/automata-network/incubator-teaclave-sgx-sdk](https://github.com/automata-network/incubator-teaclave-sgx-sdk), which is a fork of [teaclave-sgx-sdk](https://github.com/apache/incubator-teaclave-sgx-sdk) and updated to work with the latest version of the Rust toolchain and Intel SGX SDK.

## Overview

The Automata SGX SDK contains the following features which makes it easier for beginners to get started with building SGX applications:

- **Build System**: The SDK provides a build system for building SGX applications with Rust, on top of the build system, we provide the [cargo-sgx](https://crates.io/crates/cargo-sgx) tool which greatly simplifies the process of building SGX applications.
- **Remote Attestation**: The SDK implements the workflow of DCAP Remote Attestation and provides a simple interface to be integrated into your application.

### Build System

### Remote Attestation

## Getting Started

To build your first enclave, please refer to the [sgx-template](https://github.com/automata-network/sgx-template/tree/main) project. It is a good starting point to get familiar with the SDK.

### Building applications
Let's take the project structure below as an example. The `app` crate is the entrypoint and untrusted part of the application, while the `enclave` crate is the SGX enclave implementation, trusted part of the application.
<pre>
├── <b>app</b>: Entrypoint and untrusted part of the application
│ ├── <b>sgx/*</b>: Configurations for the enclave
│ ├── <b>src/main.rs</b>: Main entrypoint for the application
│ ├── <b>build.rs</b>: Builder code using the build system of Automata SGX SDK
│ └── <b>Cargo.toml</b>: Cargo.toml of the app crate
├── <b>enclave</b>: The SGX enclave implementation, trusted part of the application
│ ├── <b>lib.rs</b>: Main library file for the enclave implementation
│ └── <b>Cargo.toml</b>: Cargo.toml of the enclave crate
└── <b>Cargo.toml</b>: Cargo.toml of the workspace
</pre>

Follow the steps below to use Automata SGX SDK:

1. Update the `Cargo.toml` of the workspace to include the following dependencies, here we choose the `nightly-2024-02-01` branch.
```toml
[workspace.dependencies]
automata-sgx-sdk = { git = "https://github.com/automata-network/automata-sgx-sdk", branch = "nightly-2024-02-01" }
automata-build-script = { git = "https://github.com/automata-network/automata-sgx-sdk", branch = "nightly-2024-02-01" }
```

2. Update the `app/Cargo.toml` file as follows. 
    
    Explaination for the avaibale options of `package.metadata.sgx`:
    - `path`: Path to the enclave crate.
    - `config`: Path to the enclave configuration file.
    - `edl`: Path to the enclave EDL file.
    - `lds`: Path to the enclave LDS file.
    - `key`: Path to the enclave developer key.
    - `env`: Environment variables to be passed to the enclave.
```toml
[features]
tstd_app = ["automata-sgx-sdk/tstd_app"]

[package.metadata.sgx]
enclave = { path = "../enclave", config = "sgx/config.xml", edl = "sgx/enclave.edl", lds = "sgx/enclave.lds", key = "sgx/private.pem", env = ["MY_ENV_VAR=1"]}

[dependencies]
automata-sgx-sdk = { workspace = true }
```

3. Update the `app/src/main.rs` file to include the following code, which will call the build script to build the application.
```rust
fn main() {
    automata_build_script::build_app();
}
```

4. Update the `app/src/main.rs` file and add the `enclave!` macro.

    The `enclave!` macro is used to define the enclave and helps to initialize the enclave, it takes two arguments:
    - `name`: The name of the enclave.
    - `ecall`: The ecalls of the enclave.
```rust
automata_sgx_sdk::enclave! {
    name: MyEnclave,
    ecall: {
        fn trusted_execution() -> SgxStatus;
    }
}
```

5. Update the `enclave/Cargo.toml` file and add the following dependencies.
```toml
[features]
tstd_enclave = ["automata-sgx-sdk/tstd_enclave"]

[dependencies]
automata-sgx-sdk = { workspace = true }
```


### Generating remote attestation report
Use the `dcap_quote` function in your enclave implementation to generate a DCAP attestation report.

The `data` is the report data provided by the application, it is a 64-byte byte array, and will be included in the attestation report.
```rust
let attestation = automata_sgx_sdk::dcap::dcap_quote(data);
```

Refer to the [Automata DCAP Attestation](https://github.com/automata-network/automata-dcap-attestation) repo for more details about verification of the DCAP attestation.

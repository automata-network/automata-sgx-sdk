<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/automata-network/automata-brand-kit/main/PNG/ATA_White%20Text%20with%20Color%20Logo.png">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/automata-network/automata-brand-kit/main/PNG/ATA_Black%20Text%20with%20Color%20Logo.png">
    <img src="https://raw.githubusercontent.com/automata-network/automata-brand-kit/main/PNG/ATA_White%20Text%20with%20Color%20Logo.png" width="50%">
  </picture>
</div>

# Automata SGX SDK
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)

This repository contains the source code for the Automata SGX SDK, which is a software development kit for building secure enclaves on the Intel SGX platform. The SDK provides a set of APIs and tools to help developers build secure applications that run within an SGX enclave.

|                         | Type         | Build System     | DCAP attestation       | Rust toolchain     | Intel SGX SDK |
| ----------------------- | ------------ | ---------------- | ---------------------- | ------------------ | ------------- |
| Automata SGX SDK        | Rust SGX SDK | Cargo            | High-level abstraction | nightly-2024-02-01 | 2.24          |
| Apache Teaclave SGX SDK | Rust SGX SDK | Makefile & Cargo | Low-level abstraction  | nightly-2023-11-17 | 2.17          |
| Gramine                 | LibOS        | Makefile         | High-level abstraction | -                  | -             |

Compared to Gramine, the Automata SGX SDK provides developers with greater flexibility to design custom enclave interfaces and selectively place critical components inside the enclave. This capability enables more precise memory management and offers opportunities for performance optimization. With the help of Rust, developers can build robust, high-performance SGX applications that benefit from Rust’s strong safety guarantees and efficient memory management.

In contrast to the Apache Teaclave SGX SDK, the Automata SGX SDK simplifies the development process by removing the need for manual maintenance of Makefiles. All builds are managed through Cargo, Rust’s package manager and build system. Additionally, it supports newer versions of both the Intel SGX SDK and the Rust toolchain.


## Overview
 
The Automata SGX SDK contains the following features which makes it easier for beginners to get started with building SGX applications:

- **Build System**: The SDK provides a build system for building SGX applications with Rust, on top of the build system, we provide the [cargo-sgx](https://crates.io/crates/cargo-sgx) tool which greatly simplifies the process of building SGX applications.
- **Remote Attestation**: The SDK implements the workflow of DCAP Remote Attestation and provides a simple interface to be integrated into your application.

### Build System

The workflow of building SGX APP in the [teaclave](https://github.com/apache/incubator-teaclave-sgx-sdk/blob/v2.0.0-preview-11-17/samplecode/template/Makefile) is complicated. Overall, it includes the following steps:
* Build tstd sysroot from incubator-teaclave-sgx-sdk.
* Build the edl(Enclave Definition Language) file and generate the bridge codes.
* Build the enclave crate with custom tstd.
* Link the enclave crate with other crates and generated `<enclave_name>.so`.
* Sign the shared object with signing key, generated `<enclave_name>.signed.so`.

In Automata SGX SDK, we combine these steps on top of the cargo building system automatically, so you can get rid of the hassle of modifying the Makefile.

### DCAP Remote Attestation

As the most important feature of the SGX APP, we have built the DCAP attestation generation function based on [SGXDataCenterAttestationPrimitives](https://github.com/automata-network/SGXDataCenterAttestationPrimitives).   
Users do not need to understand what's the ocalls during this process.

## Supported Environment

| Operator System  | Intel SGX SDK | Rust Toolchain     | Status |
| ---------------- | ------------- | ------------------ | ------ |
| Ubuntu 20.04 LTS | 2.24          | nightly-2024-02-01 | ✅      |
| Ubuntu 22.04 LTS | 2.24          | nightly-2024-02-01 | ✅      |

## Getting Started

To build your first enclave, please refer to the [sgx-scaffold](https://github.com/automata-network/sgx-scaffold/tree/main) project. It is a good starting point to get familiar with the SDK. 

### Building applications
Let's take the project structure below as an example. The `app` crate is the entrypoint and untrusted part of the application, while the `enclave` crate is the SGX enclave implementation, trusted part of the application.
<pre>
├── <b>app</b>: Entrypoint and untrusted part of the application
│ ├── <b>sgx/*</b>: Configurations for the enclave
│ ├── <b>src/main.rs</b>: Main entrypoint for the application
│ ├── <b>build.rs</b>: Builder code using the build system of Automata SGX SDK
│ └── <b>Cargo.toml</b>: Cargo.toml of the app crate
├── <b>enclave</b>: The SGX enclave implementation, trusted part of the application
│ ├── <b>src/lib.rs</b>: Main library file for the enclave implementation
│ └── <b>Cargo.toml</b>: Cargo.toml of the enclave crate
└── <b>Cargo.toml</b>: Cargo.toml of the workspace
</pre>

Follow the steps below to use Automata SGX SDK:

1. Specify the rust-toolchain, please ensure that the Rust version used aligns with the [Automata SGX SDK](Cargo.toml#L10). We will use `nightly-2024-02-01` in this case.

    ```bash
    echo 'nightly-2024-02-01' > rust-toolchain
    ```

2. Update the `Cargo.toml` of the workspace to include the following dependencies, here we choose the `main` branch.
    ```toml
    [workspace.dependencies]
    automata-sgx-sdk = { git = "https://github.com/automata-network/automata-sgx-sdk", branch = "main" }
    automata-build-script = { git = "https://github.com/automata-network/automata-sgx-sdk", branch = "main" }
    ```

3. Update the `app/Cargo.toml` file as follows. 
    
    Explaination for the avaibale options of `package.metadata.sgx`:
    - `path`: (required) Path to the enclave crate.
    - `config`: (required) Path to the enclave configuration file.
    - `edl`: (required) Path to the enclave EDL file.
    - `lds`: (required) Path to the enclave LDS file.
    - `key`: (required) Path to the enclave developer key.
    - `env`: (optional) Environment variables to be passed to the enclave builder.

    ```toml
    [features]
    tstd_app = ["automata-sgx-sdk/tstd_app"]

    [package.metadata.sgx]
    my_enclave = { path = "../enclave", config = "sgx/config.xml", edl = "sgx/enclave.edl", lds = "sgx/enclave.lds", key = "sgx/private.pem", env = ["MY_ENV_VAR=1"] }

    [dependencies]
    automata-sgx-sdk = { workspace = true }
    ```

4. Update the `app/src/main.rs` file to include the following code, which will call the build script to build the application.
    ```rust
    fn main() {
        automata_build_script::build_app();
    }
    ```

5. Update the `app/src/main.rs` file and add the `enclave!` macro.

    The `enclave!` macro is used to define the enclave and helps to initialize the enclave, it takes two arguments:
    - `name`: The name of the enclave. **The name needs to align with `package.metadata.sgx`, but it should be converted from snake_case to CamelCase with the first letter capitalized**. In this case we use `MyEnclave` instead of `my_enclave`.
    - `ecall`: The ecalls of the enclave.

    ```rust
    automata_sgx_sdk::enclave! {
        name: MyEnclave,
        ecall: {
            fn trusted_execution() -> SgxStatus;
        }
    }
    ```

6. Update the `enclave/Cargo.toml` file and add the following dependencies. The `lib.name` should be same as the name defined on `package.metadata.sgx`.

    ```toml
    [lib]
    name = "my_enclave"
    crate-type = ["staticlib"]

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

## Projects using Automata SGX SDK

* [sgx-prover](https://github.com/automata-network/sgx-prover): A prover that supports to execute Scroll and Linea blocks in SGX enclave and generate the PoE (Proof of Execution).
* [sgx-scallfold](https://github.com/automata-network/sgx-scaffold): A scaffold for creating an SGX enclave with Rust.
* [sgx-revm](https://github.com/automata-network/revm-sgx): A PoC that embeded revm inside Intel SGX enclave.

## Acknowledgements
- [incubator-teaclave-sgx-sdk](https://github.com/apache/incubator-teaclave-sgx-sdk): The Automata SGX SDK is built on top of [https://github.com/automata-network/incubator-teaclave-sgx-sdk](https://github.com/automata-network/incubator-teaclave-sgx-sdk), which is a fork of `incubator-teaclave-sgx-sdk` and updated to work with the latest version of the Rust toolchain and Intel SGX SDK.

## Disclaimer
This project is under development. All source code and features are not production ready.
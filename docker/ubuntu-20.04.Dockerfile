FROM ubuntu:20.04 as builder

LABEL automata.rust_toolchain="nightly-2024-02-01"
LABEL automata.intel_sgx_sdk="2.24.100.3"
LABEL org.opencontainers.image.description="Automata SGX SDK Base Image for Ubuntu 20.04"

ENV DEBIAN_FRONTEND=noninteractive

RUN chmod 1777 /tmp && \
    apt update && \
    apt install -y \
        unzip lsb-release debhelper cmake reprepro autoconf automake bison build-essential curl dpkg-dev expect flex gcc-8 gdb \
        git git-core gnupg kmod libboost-system-dev libboost-thread-dev libcurl4-openssl-dev libiptcdata0-dev libjsoncpp-dev \
        liblog4cpp5-dev libprotobuf-dev libssl-dev libtool libxml2-dev ocaml ocamlbuild protobuf-compiler python-is-python3 \
        texinfo uuid-dev vim wget software-properties-common clang perl pkgconf libboost-dev libsystemd0 nlohmann-json3-dev

RUN rm -rf /var/lib/apt/lists/*

ENV rust_toolchain nightly-2024-02-01
RUN cd /root && \
    curl 'https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init' --output /root/rustup-init && \
    chmod +x /root/rustup-init && \
    echo '1' | /root/rustup-init --default-toolchain ${rust_toolchain} --profile minimal && \
    echo 'source /root/.cargo/env' >> /root/.bashrc && \
    rm /root/rustup-init && rm -rf /root/.cargo/registry && rm -rf /root/.cargo/git

ENV VERSION                 2.24.100.3-focal1
ENV DCAP_VERSION            1.21.100.3-focal1
ENV SDK_URL="https://download.01.org/intel-sgx/sgx-linux/2.24/distro/ubuntu20.04-server/sgx_linux_x64_sdk_2.24.100.3.bin"
ENV LOCAL_REPO="https://download.01.org/intel-sgx/sgx-linux/2.24/distro/ubuntu20.04-server/sgx_debian_local_repo.tgz"

RUN cd /root && \
    wget https://download.01.org/intel-sgx/sgx-linux/2.24/as.ld.objdump.r4.tar.gz && \
    tar xzf as.ld.objdump.r4.tar.gz && \
    cp -r external/toolset/ubuntu20.04/* /usr/bin/ && \
    rm -rf ./external ./as.ld.objdump.r4.tar.gz

RUN cd /opt && \
    curl -LO $LOCAL_REPO && \
    tar zxvf sgx_debian_local_repo.tgz && \
    echo "deb [trusted=yes] file:/opt/sgx_debian_local_repo focal main" | tee /etc/apt/sources.list.d/intel-sgx.list

RUN cd /root && \
    curl -o sdk.sh $SDK_URL && \
    chmod a+x /root/sdk.sh && \
    echo -e 'no\n/opt' | ./sdk.sh && \
    echo 'source /opt/sgxsdk/environment' >> /root/.bashrc && \
    cd /root && \
    rm ./sdk.sh

RUN apt-get update && \
    apt-get install -y \
        libsgx-headers=$VERSION \
        libsgx-ae-epid=$VERSION \
        libsgx-ae-le=$VERSION \
        libsgx-ae-pce=$VERSION \
        libsgx-aesm-ecdsa-plugin=$VERSION \
        libsgx-aesm-epid-plugin=$VERSION \
        libsgx-aesm-launch-plugin=$VERSION \
        libsgx-aesm-pce-plugin=$VERSION \
        libsgx-aesm-quote-ex-plugin=$VERSION \
        libsgx-enclave-common=$VERSION \
        libsgx-enclave-common-dev=$VERSION \
        libsgx-epid=$VERSION \
        libsgx-epid-dev=$VERSION \
        libsgx-launch=$VERSION \
        libsgx-launch-dev=$VERSION \
        libsgx-quote-ex=$VERSION \
        libsgx-quote-ex-dev=$VERSION \
        libsgx-uae-service=$VERSION \
        libsgx-urts=$VERSION \
        sgx-aesm-service=$VERSION \
        libsgx-dcap-ql=$DCAP_VERSION \
        libsgx-dcap-ql-dev=$DCAP_VERSION \
        libsgx-dcap-quote-verify=$DCAP_VERSION \
        libsgx-dcap-quote-verify-dev=$DCAP_VERSION \
        libsgx-dcap-default-qpl=$DCAP_VERSION \
        libsgx-dcap-default-qpl-dev=$DCAP_VERSION \
        libsgx-ae-qve=$DCAP_VERSION \
        libsgx-ae-qe3=$DCAP_VERSION \
        libsgx-pce-logic=$DCAP_VERSION \
        libsgx-qe3-logic=$DCAP_VERSION \
        libsgx-ra-network=$DCAP_VERSION \
        libsgx-ra-uefi=$DCAP_VERSION && \
    mkdir /var/run/aesmd && \
    rm -rf /var/lib/apt/lists/* && \
    rm -rf /var/cache/apt/archives/*

RUN cd /root && \
    git clone https://github.com/automata-network/Azure-DCAP-Client -b 20.04_1.12.3 && \
    cd Azure-DCAP-Client/src/Linux && \
    ./configure && \
    make && \
    make install && \
    rm -f /usr/lib/x86_64-linux-gnu/libdcap_quoteprov.so* && cp /usr/local/lib/libdcap_quoteprov.so /usr/lib/x86_64-linux-gnu

ENV SGX_SDK='/opt/sgxsdk'
ENV LD_LIBRARY_PATH=/usr/lib:/usr/local/lib
ENV LD_RUN_PATH=/usr/lib:/usr/local/lib
ENV LD_LIBRARY_PATH="$LD_LIBRARY_PATH:/opt/sgxsdk/sdk_libs"
ENV RUSTFLAGS='-L $SGX_SDK/lib64/'
ENV PATH='/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/root/.cargo/bin'
ENV PKG_CONFIG_PATH='$SGX_SDK/pkgconfig'

RUN --mount=type=cache,target=/root/.cargo/registry/index \
    --mount=type=cache,target=/root/.cargo/registry/cache \
    --mount=type=cache,target=/root/.cargo/git \
    rustup component add rust-src --toolchain ${rust_toolchain}-x86_64-unknown-linux-gnu
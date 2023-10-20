ARG RUST_TOOLCHAIN

FROM washanhanzi/databend-build-tools:base-${RUST_TOOLCHAIN}

RUN echo $(arch)
RUN echo $(uname -m)
ARG ARCH
COPY rust-tools.txt /build/scripts/setup/rust-tools.txt
RUN /build/scripts/setup/dev_setup.sh -ycd && \
    rm -rf /opt/rust/cargo/git && \
    rm -rf /opt/rust/cargo/registry && \
    rm -rf /root/.cache/*

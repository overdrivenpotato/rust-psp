FROM psptoolchain as psptoolchain

FROM rust:latest as builder
COPY --from=psptoolchain /usr/local/pspdev/ /usr/local/pspdev/
RUN rustup set profile minimal
RUN rustup default nightly
RUN cargo install xargo
RUN rustup component add rust-src
ENV PATH="/usr/local/pspdev/bin:${PATH}"

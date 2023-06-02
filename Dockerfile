FROM --platform=$BUILDPLATFORM rust:latest as build
WORKDIR /op-challenger
ARG TARGETARCH

COPY ./platform.sh .
RUN chmod +x ./platform.sh
RUN ./platform.sh
RUN rustup target add $(cat /.platform) 
RUN apt-get update && apt-get install -y $(cat /.compiler)
ENV CC_x86_64_unknown_linux_musl=x86_64-linux-gnu-gcc

COPY ./ ./
RUN RUSTFLAGS="$(cat /.rustflags)" cargo build --release --config net.git-fetch-with-cli=true --target $(cat /.platform)
RUN cp /op-challenger/target/$(cat /.platform)/release/op-challenger /op-challenger/op-challenger

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=build /op-challenger/op-challenger /usr/local/bin

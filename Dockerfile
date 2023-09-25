# Use a Rust base image
FROM rust:latest as build

# Install diesel-cli
RUN cargo install diesel_cli --no-default-features --features postgres

# Update the package list and install protoc
RUN apt-get update && apt-get install -y unzip curl \
    && PROTOC_ZIP=protoc-24.3-linux-aarch_64.zip \
    && curl -OL https://github.com/google/protobuf/releases/download/v24.3/$PROTOC_ZIP \
    && unzip -o $PROTOC_ZIP -d /usr/local bin/protoc \
    && rm -f $PROTOC_ZIP


COPY .. /root/identity_service

WORKDIR /root/identity_service

RUN cargo build --release

FROM ubuntu:22.04 as final
EXPOSE 8002

# Copy server binary and diesel binary and migrations
COPY --from=build /root/identity_service/target/release/server /bin/server
COPY --from=build /usr/local/cargo/bin/diesel /bin/diesel
COPY --from=build /root/identity_service/src/db/migrations /migrations

# Set the PATH for diesel
ENV PATH="/bin:${PATH}"

RUN apt-get update && apt-get install libpq5 -y
RUN chmod +x /bin/server
CMD ["/bin/server"]
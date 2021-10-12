# Create the build container to compile the hello world program
FROM ekidd/rust-musl-builder:latest as builder
ADD --chown=rust:rust . ./
RUN cargo build --release --target=x86_64-unknown-linux-musl --bin server

# Create the execution container by copying the compiled hello world to it and running it
FROM scratch

# comment the following FROM and RUN lines if you need the health checking
# FROM alpine
# # Download the gRPC health probe
# RUN GRPC_HEALTH_PROBE_VERSION=v0.4.5 && \
#     wget -qO/bin/grpc_health_probe https://github.com/grpc-ecosystem/grpc-health-probe/releases/download/${GRPC_HEALTH_PROBE_VERSION}/grpc_health_probe-linux-amd64 && \
#     chmod +x /bin/grpc_health_probe

COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/server /usr/local/bin/server
EXPOSE 5051
EXPOSE 5052
CMD ["/usr/local/bin/server"]

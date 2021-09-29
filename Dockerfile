# Create the build container to compile the hello world program
FROM ekidd/rust-musl-builder:latest as builder
ADD --chown=rust:rust . ./
RUN cargo build --release --target=x86_64-unknown-linux-musl --bin server

# Create the execution container by copying the compiled hello world to it and running it
FROM scratch
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/server /usr/local/bin/server
EXPOSE 5051
EXPOSE 5052
CMD ["/usr/local/bin/server"]

FROM gcr.io/distroless/static
COPY target/x86_64-unknown-linux-musl/release/stubr /
ENTRYPOINT ["./stubr"]
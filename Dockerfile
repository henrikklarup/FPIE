FROM alpine:3.10
COPY ./artifacts/x86_64-unknown-linux-musl/fpie /fpie
ENTRYPOINT ["/fpie"]

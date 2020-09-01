FROM scratch
COPY ./artifacts/x86_64-unknown-linux-musl/fpie /fpie
ENTRYPOINT ["/fpie"]

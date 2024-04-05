FROM ubuntu:22.04
COPY ./target/release/accounts-integration ./target/release/accounts-integration
ENTRYPOINT ["./target/release/accounts-integration"]
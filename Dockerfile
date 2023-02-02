FROM rust:slim
COPY ./target/release/accounts-integration ./target/release/accounts-integration
ENTRYPOINT ["./target/release/accounts-integration"]
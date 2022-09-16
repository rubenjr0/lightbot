FROM rust as builder
COPY . .
RUN cargo build --release

FROM rust as runtime
COPY --from=builder target/release/lightbot lightbot

CMD ["./lightbot"]

FROM rust:1.59 as builder

COPY . /src

WORKDIR /src

RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /src/target/release/grading-runner /app/

CMD [ "/app/grading-runner" ]
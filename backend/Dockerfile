FROM rust:1.59 as builder

COPY . /src

WORKDIR /src/grading-scheduler

RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=builder /src/target/release/grading-scheduler /app/

CMD [ "/app/grading-scheduler" ]

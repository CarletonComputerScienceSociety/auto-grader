FROM rust:1.59 as builder

COPY . /src

WORKDIR /src

RUN cargo build --release

FROM debian:bullseye
COPY --from=builder /src/target/release/grading-runner /app/

# Install updates
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        git \
        make \
        # C/C++
        gcc \
        g++ \
        # Python
        python \
        python-dev \
        # Java
        openjdk-11-jdk

COPY tests /tests

CMD [ "/app/grading-runner" ]

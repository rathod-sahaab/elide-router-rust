FROM rust:latest as build

COPY . .

RUN mkdir -p /app

RUN cargo build --release

RUN cp target/release/elide-router /app

FROM ubuntu:latest

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get -y update && \
     apt-get -y upgrade  && \
     apt -y install ca-certificates libssl-dev libpq-dev

COPY --from=build /app/elide-router /usr/local/bin

ENTRYPOINT ["elide-router"]

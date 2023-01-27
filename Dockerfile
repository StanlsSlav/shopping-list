FROM postgres AS base

FROM rust AS builder
RUN cargo install diesel_cli --no-default-features --features postgres
RUN diesel setup

CMD cargo run

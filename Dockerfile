FROM rust:nightly

ENV DATABASE_URL=mysql://root:V422w0rt@10.10.20.57/db_hrm
ENV DATABASE_URL2=mysql://admhrm:r4h4s14@10.10.20.35/db_trf_tukin
ENV DATABASE_URL3=mysql://root:V422w0rt@10.10.20.57/db_sispedap
ENV RUST_BACKTRACE=1

RUN USER=root cargo new --bin rust-kinerja-service
WORKDIR /rust-kinerja-service
COPY ./Cargo.toml ./Cargo.toml
#RUN cargo build
RUN cargo build --release

RUN rm src/*.rs
COPY ./src ./src
COPY ./assets ./assets
RUN rm ./target/release/deps/rust_kinerja_service*
RUN cargo build --release

CMD ["./target/release/rust-kinerja-service"]

EXPOSE 8089

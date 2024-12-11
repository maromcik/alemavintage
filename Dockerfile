FROM rust:1.75

WORKDIR /usr/src/alemavintage

RUN apt-get update
RUN apt-get install -y postgresql-client zip build-essential

ENV SQLX_OFFLINE=true
COPY ./.sqlx ./.sqlx
COPY ./migrations ./migrations
COPY ./proto ./proto
COPY ./src ./src
COPY ./templates ./templates
COPY ./.env ./env
COPY ./build.rs ./build.rs
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml


EXPOSE 8000

RUN cargo install --path .

CMD ["alemavintage"]
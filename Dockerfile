FROM rust:1.83

WORKDIR /usr/src/alemavintage
RUN mkdir ./media

RUN apt-get update
RUN apt-get install -y postgresql-client zip build-essential autoconf libtool pkg-config libgexiv2-dev

ENV SQLX_OFFLINE=true
COPY ./.env ./.env
COPY ./.sqlx ./.sqlx
COPY ./migrations ./migrations
COPY ./src ./src
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo install --path .

COPY ./static ./static
COPY ./templates ./templates
COPY ./media ./media

EXPOSE 8000

CMD ["alemavintage"]
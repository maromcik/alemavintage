FROM rust:1.83

WORKDIR /usr/src/alemavintage

RUN apt-get update
RUN apt-get install -y postgresql-client zip build-essential autoconf libtool pkg-config libgexiv2-dev

ENV SQLX_OFFLINE=true
COPY ./.env ./.env
COPY ./.sqlx ./.sqlx
COPY ./media ./media
COPY ./migrations ./migrations
COPY ./src ./src
COPY ./static ./static
COPY ./templates ./templates
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml


EXPOSE 8000

RUN cargo install --path .

CMD ["alemavintage"]
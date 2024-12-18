FROM rust:1.83

WORKDIR /usr/src/alemavintage

RUN apt-get update
RUN apt-get install -y libgexiv2-dev

ENV SQLX_OFFLINE=true
COPY ./.sqlx ./.sqlx
COPY ./migrations ./migrations
COPY ./src ./src
COPY ./static ./static
COPY ./templates ./templates
COPY ./.env ./env
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml


EXPOSE 8000

RUN cargo install --path .

CMD ["alemavintage"]
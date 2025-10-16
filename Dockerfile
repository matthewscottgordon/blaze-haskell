FROM rust:1.90

COPY . /usr/app
WORKDIR /usr/app

RUN cargo install --path .

CMD ["blaze-haskell"]

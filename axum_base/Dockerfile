FROM rust:1.80

WORKDIR /app
COPY . .

RUN cargo install --path .

EXPOSE 18080

CMD ["server"]
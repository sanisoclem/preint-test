FROM scratch
COPY ./target/x86_64-unknown-linux-musl/release/rustaroo-api /app
WORKDIR /
EXPOSE 8000
ENTRYPOINT ["./app"]
FROM clux/muslrust:nightly-2020-02-22 as build
WORKDIR /tmp/inspekt
COPY . /tmp/inspekt
RUN cargo build --release

FROM alpine:3.9
WORKDIR /root/
COPY --from=build /tmp/inspekt/target/x86_64-unknown-linux-musl/release/inspekt .
COPY --from=build /tmp/inspekt/templates .

ENV ROCKET_ENV production
EXPOSE 8000
CMD ["./inspekt"]

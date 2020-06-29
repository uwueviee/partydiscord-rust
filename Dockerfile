FROM rust

COPY ./ ./

RUN cargo build --target x86_64-unknown-linux-gnu --release

RUN mkdir -p /build-out

RUN cp target/x86_64-unknown-linux-gnu/release/partydiscord-rust /build-out/

RUN ls /build-out/

FROM alpine

CMD ["/build-out/partydiscord-rust"]
FROM rust

COPY ./ ./

RUN cargo build --target x86_64-unknown-linux-musl --release

RUN mkdir -p /build-out

RUN cp target/x86_64-unknown-linux-musl/release/partydiscord-rust /build-out/

RUN ls /build-out/

FROM scratch

COPY --from=build /build-out/partydiscord-rust /

CMD ["/partydiscord-rust"]
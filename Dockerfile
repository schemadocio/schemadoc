FROM rust:alpine3.16 as builder

RUN apk add -q --update-cache --no-cache build-base openssl-dev

WORKDIR /app

COPY . .

# https://users.rust-lang.org/t/sigsegv-with-program-linked-against-openssl-in-an-alpine-container/52172/3
ENV RUSTFLAGS="-C target-feature=-crt-static"

RUN cargo build

FROM alpine:3.16

COPY --from=builder /app/target/debug/schemadoc /bin/schemadoc

WORKDIR /schemadoc

# Error loading shared library libgcc_s.so.1: No such file or directory
RUN apk update --quiet \
        && apk add -q --no-cache tini libgcc

RUN adduser -D schemadoc
USER schemadoc

ENTRYPOINT ["tini", "--", "/bin/schemadoc"]
CMD ["serve", "--schedule"]
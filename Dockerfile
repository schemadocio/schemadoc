FROM rust:alpine3.18 as builder

RUN apk add -q --update-cache --no-cache build-base openssl-dev

WORKDIR /server

COPY . .

# https://users.rust-lang.org/t/sigsegv-with-program-linked-against-openssl-in-an-alpine-container/52172/3
ENV RUSTFLAGS="-C target-feature=-crt-static"

RUN cargo build --release

FROM node:18.12 as builder-ui

WORKDIR /ui

COPY ./ui/package.json ./ui/package-lock.json ./

RUN npm install

COPY ./ui .

RUN npm run build

FROM alpine:3.18

ENV SD_PERSISTENCE_PATH=/schemadoc
ENV SD_PERSISTENCE_CONFIG_PATH=/schemadoc
ENV SD_FRONTEND_STATIC_FILES=/static

COPY --from=builder /server/target/release/schemadoc /bin/schemadoc
COPY --from=builder-ui /ui/build /static

WORKDIR /schemadoc

# Error loading shared library libgcc_s.so.1: No such file or directory
RUN apk update --quiet \
    && apk add -q --no-cache tini libgcc

RUN adduser -D schemadoc
USER schemadoc

EXPOSE 9753

ENTRYPOINT ["tini", "--", "/bin/schemadoc"]
# Serve with scheduler running in the same process
CMD ["serve", "--schedule"]
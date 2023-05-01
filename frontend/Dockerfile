FROM node:hydrogen-alpine3.16 AS node

FROM rust:1.65.0-alpine3.16 AS build
WORKDIR /app

# Get node from node alpine
COPY --from=node /usr/lib /usr/lib
COPY --from=node /usr/local/share /usr/local/share
COPY --from=node /usr/local/lib /usr/local/lib
COPY --from=node /usr/local/include /usr/local/include
COPY --from=node /usr/local/bin /usr/local/bin

# Install system requirements
RUN apk upgrade --update-cache --available && \
    apk add openssl build-base pkgconfig openssl-dev
ARG OPENSSL_DIR=/usr
RUN cargo install simple-http-server --root /app
RUN rustup target add wasm32-unknown-unknown

# Actually build the app
COPY ./ ./
COPY ./frontend ./frontend
# RUN cd frontend && npm install -g pnpm && pnpm install && pnpm run build && cd /app
RUN npm install -g pnpm
RUN npm install
RUN pnpm install
RUN npm install -g webpack
RUN pnpm install -D webpack-cli
RUN npm install -g wasm-pack
RUN cd frontend && pnpm install -D webpack-cli
RUN cd frontend && pnpm run build

COPY ./database ./database
COPY ./shared ./shared
COPY docker.env ./frontend/.env

FROM alpine:3.16
WORKDIR /app
COPY --from=build /app/frontend/dist ./static
COPY --from=build /app/bin/simple-http-server .
RUN chmod +x ./simple-http-server
EXPOSE 8000
CMD ["./simple-http-server", "./static", "-i", "-p", "8000", "--nocache", "--try-file", "./static/index.html"]

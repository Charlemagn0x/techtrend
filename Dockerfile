FROM rust:1.56 as builder
WORKDIR /usr/src/techtrend
COPY . .
RUN cargo install --path .

FROM node:14 as frontend
WORKDIR /app
COPY --from=builder /usr/src/techtrend/frontend .
RUN npm install
RUN npm run build

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/techtrend /usr/local/bin/techtrend
COPY --from=frontend /app/build /var/www/html
EXPOSE 8080

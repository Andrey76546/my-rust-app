# ---- Этап 1: сборка приложения ----
FROM rust:1-slim AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -f target/release/my-rust-app*

COPY src ./src
# Фиктивная сборка оставляет fingerprint Cargo; обновление времени файла
# гарантирует пересборку настоящего main.rs.
RUN touch src/main.rs && cargo build --release

# ---- Этап 2: минимальный образ для запуска ----
FROM debian:stable-slim

RUN useradd --create-home appuser
WORKDIR /home/appuser
COPY --from=builder /app/target/release/my-rust-app .
USER appuser

CMD ["./my-rust-app"]


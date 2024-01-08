FROM rust:latest AS builder
WORKDIR /build
COPY . .
RUN cargo install --path . --root /app

FROM fedora:latest
RUN dnf install pandoc texlive-scheme-basic -y
COPY --from=builder /app/bin/clanko_bot /app/clanko_bot
WORKDIR /app
CMD ["./clanko_bot"]

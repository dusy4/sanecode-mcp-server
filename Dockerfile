# The demo host over stdio (made-up accounts; nothing on the host changes), for MCP inspectors.
#   docker build -t sanecode-mcp-demo .
#   docker run -i --rm sanecode-mcp-demo
FROM rust:1-slim AS build
WORKDIR /src
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY tools.json ./
RUN cargo build --release --locked --bin sanecode-mcp-demo

FROM debian:bookworm-slim
COPY --from=build /src/target/release/sanecode-mcp-demo /usr/local/bin/sanecode-mcp-demo
ENTRYPOINT ["/usr/local/bin/sanecode-mcp-demo"]

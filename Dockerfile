FROM rust
WORKDIR /app
COPY . /app

RUN apt update -y
RUN apt install build-essential llvm lld libclang-dev -y
RUN apt install curl -y
# Install Node.js and npm
RUN curl -sL https://deb.nodesource.com/setup_16.x | bash - \
    && apt-get install -y nodejs

# Postgresql client for diesel
RUN apt install postgresql-client -y

WORKDIR /app/client
RUN npm install
RUN npm run build

WORKDIR /app
RUN cargo install diesel_cli --no-default-features --features postgres
ENV PATH="/root/.cargo/bin:$PATH"

RUN cargo build --release

EXPOSE 8080

ENTRYPOINT ["/bin/sh", "-c" , "diesel setup && /app/target/release/movie-rama"]


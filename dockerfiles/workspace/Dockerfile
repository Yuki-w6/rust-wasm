FROM rust:1.75

WORKDIR /mnt

RUN apt-get update && \
    apt-get install -y \
    bash \
    git \
    bash-completion \
    && rm -rf /var/lib/apt/lists/*

COPY . .

RUN cargo install cargo-compete

# bash をデフォルトシェルとして設定
SHELL ["/bin/bash", "-c"]

CMD ["tail", "-f", "/dev/null"]

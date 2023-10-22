FROM rust:1.70

WORKDIR /usr/src/rust_study

RUN apt-get -y update && apt-get upgrade -qqy && apt-get -y install \
    bash \
    git 

COPY . .
RUN cargo install cargo-compete

CMD ["tail", "-f", "/dev/null"]

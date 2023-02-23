FROM rust:1.63

RUN apt-get -y update && apt-get install -y

COPY . /code
WORKDIR /code

RUN cargo install --path .

RUN chmod 755 test.sh

FROM rust:1.63

RUN apt-get -y update && apt-get install -y dos2unix

COPY . /code
WORKDIR /code

RUN cargo install --path .

RUN dos2unix test.sh
RUN chmod 755 test.sh

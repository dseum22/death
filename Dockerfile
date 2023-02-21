FROM ubuntu:latest

RUN apt-get -y update && apt-get install -y clang make g++

COPY . /code
WORKDIR /code

# RUN clang++ -o main main.cc
RUN make

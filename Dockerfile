FROM rust:latest

WORKDIR /usr/src/demon
#COPY . .
RUN apt-get update -y
RUN apt-get install -y linux-headers-amd64 libnl-genl-3-dev && apt-get install -y lvm-3.9-dev libclang-3.9-dev clang-3.9 libnl-utils
#RUN cargo install
CMD [ "demon" ]

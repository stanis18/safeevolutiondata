FROM ubuntu:18.04

RUN apt update
RUN apt install -y git curl
RUN apt update
RUN curl -fsSL https://deb.nodesource.com/setup_16.x | bash -
RUN apt install -y nodejs
RUN corepack enable
RUN mkdir /home/front
RUN mkdir /home/back

RUN apt install docker.io -y
RUN docker --version



RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN apt install gcc -y
RUN apt install pkg-config libssl-dev -y

COPY ./verification-tool /home/back
RUN cd /home/back && cargo build

COPY ./front-verification-tool /home/front
RUN cd /home/front && yarn install

EXPOSE 3000

COPY run.sh /home/

RUN chmod +x ./home/run.sh

CMD ./home/run.sh

FROM ubuntu:18.04

# install rust
RUN apt-get update -y && \
    apt-get upgrade -y
RUN apt-get install -y curl
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH $PATH:$HOME/.cargo/bin/

# setting for cross-compile to windows
RUN apt-get install -y gcc gcc-mingw-w64
RUN $HOME/.cargo/bin/rustup target add x86_64-pc-windows-gnu

# sdl2
## ubuntu
RUN apt-get install -y libsdl2-dev
WORKDIR /tmp
## windows
RUN curl -OL https://www.libsdl.org/release/SDL2-devel-2.0.10-mingw.tar.gz && \
    tar zxvf SDL2-devel-2.0.10-mingw.tar.gz && \
    cp -r SDL2-2.0.10/x86_64-w64-mingw32/lib/* ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-pc-windows-gnu/lib/

WORKDIR /root/src
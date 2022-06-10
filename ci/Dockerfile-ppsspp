FROM ubuntu:latest as ppsspp

ENV DEBIAN_FRONTEND="noninteractive"

RUN apt-get -y update
RUN apt-get install -y build-essential cmake git libsdl2-dev python libglew-dev
RUN git clone https://github.com/hrydgard/ppsspp --recursive
WORKDIR /ppsspp/build-sdl
RUN cmake .. \
    -DCMAKE_BUILD_TYPE=Release \
    -DCMAKE_SKIP_RPATH=ON \
    -DHEADLESS=ON \
    -DUSE_SYSTEM_LIBZIP=ON
RUN make
RUN make install

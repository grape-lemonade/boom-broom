FROM ghcr.io/cross-rs/x86_64-pc-windows-gnu:latest

RUN apt-get install --assume-yes wget  && \
    wget https://github.com/libsdl-org/SDL/releases/download/release-2.26.1/SDL2-devel-2.26.1-mingw.tar.gz && \
    wget https://github.com/libsdl-org/SDL_image/releases/download/release-2.6.2/SDL2_image-devel-2.6.2-mingw.tar.gz && \
    tar -xvzf SDL2-devel-2.26.1-mingw.tar.gz && \
    tar -xvzf SDL2_image-devel-2.6.2-mingw.tar.gz && \
	ln -s /usr/bin/x86_64-w64-mingw32 /usr/local/x86_64-w64-mingw32 && \
    cd SDL2-2.26.1 && \
	make cross && \
	cd ../SDL2_image-2.6.2 && \
	make cross

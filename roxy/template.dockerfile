FROM amd64/ubuntu:22.04

ENV DEBIAN_FRONTEND noninteractive
ENV TZ Asia/Tokyo
ENV LLVM_VERSION=15
ENV GCC_VERSION=11

RUN mkdir /root/tools
RUN mkdir /root/workspace
RUN mkdir /root/.config

RUN echo hello

RUN apt-get -y clean
RUN apt-get -y update
RUN apt-get -y full-upgrade
RUN apt-get -y install make \
    cmake \
    automake \
    meson \
    ninja-build \
    flex \
    hyperfine \
    git \
    xz-utils \
    bzip2 \
    wget \
    jupp \
    nano \
    bash-completion \
    less \
    vim \
    joe \
    ssh \
    psmisc \
    python3 \
    python3-dev \
    python3-pip \
    python-is-python3 \
    libtool libtool-bin libglib2.0-dev \
    apt-transport-https gnupg dialog \
    gnuplot-nox libpixman-1-dev bc \
    gcc-${GCC_VERSION} g++-${GCC_VERSION} gcc-${GCC_VERSION}-plugin-dev gdb lcov \
    clang-${LLVM_VERSION} \
    lld-${LLVM_VERSION} lldb-${LLVM_VERSION} llvm-${LLVM_VERSION} \
    llvm-${LLVM_VERSION}-dev llvm-${LLVM_VERSION}-runtime llvm-${LLVM_VERSION}-tools \
    $([ "$(dpkg --print-architecture)" = "amd64" ] && echo gcc-${GCC_VERSION}-multilib gcc-multilib) \
    $([ "$(dpkg --print-architecture)" = "arm64" ] && echo libcapstone-dev)

RUN apt-get -y install ca-certificates \
    musl-tools \
    libssl-dev \
    zlib1g-dev \
    libbz2-dev \
    libreadline-dev \
    libsqlite3-dev \
    curl \
    zip \
    unzip \
    libncurses5-dev \
    libncursesw5-dev \
    tk-dev \
    libxml2-dev \
    libxmlsec1-dev \
    libffi-dev \
    liblzma-dev \
    libyaml-dev \
    tree \
    neofetch \
    openssh-server \
    patchelf \
    elfutils \
    file \
    devscripts \
    fish \
    libjpeg-dev \
    autogen \
    autoconf \
    texinfo \
    libgmp-dev \
    libmpfr-dev \
    libasound2-dev \
    libflac-dev \
    libogg-dev \
    libvorbis-dev \
    libopus-dev \
    pkg-config \
    libc6-dev \
    libfreetype6-dev \
    ltrace \
    afl \
    m4 \
    libseccomp-dev libseccomp2 seccomp \
    ruby-dev \
    gdb \
    ripgrep \
    gdb-multiarch \
    expect \
    gdbserver

RUN rm -rf /var/lib/apt/lists/*

RUN chsh -s /bin/fish
RUN mkdir /root/.config/fish

# pyenv
RUN git clone https://github.com/pyenv/pyenv.git $HOME/.pyenv
RUN echo 'set -x PYENV_ROOT /root/.pyenv' >> /root/.config/fish/config.fish
RUN echo 'set -x PATH  /root/.pyenv/bin $PATH' >> /root/.config/fish/config.fish
RUN echo 'set -x PATH /root/.pyenv/shims $PATH' >> /root/.config/fish/config.fish
RUN /root/.pyenv/bin/pyenv install 3.10.13
RUN /root/.pyenv/bin/pyenv global 3.10.13
ENV PATH $PATH:/root/.pyenv/shims/

# python tools
RUN /root/.pyenv/shims/pip install ptrlib
RUN /root/.pyenv/shims/pip install pwntools
RUN /root/.pyenv/shims/pip install bpython

# rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN echo 'set -x PATH /root/.cargo/bin $PATH' >> /root/.config/fish/config.fish
ENV PATH $PATH:/root/.cargo/bin

RUN ionice -c2 -n7 taskset -c 0-6 nice -n 19 cargo install ropr
RUN ionice -c2 -n7 taskset -c 0-6 nice -n 19 cargo install bat
RUN ionice -c2 -n7 taskset -c 0-6 nice -n 19 cargo install eza
RUN ionice -c2 -n7 taskset -c 0-6 nice -n 19 cargo install fd-find
RUN ionice -c2 -n7 taskset -c 0-6 nice -n 19 cargo install pwninit
RUN ionice -c2 -n7 taskset -c 0-6 nice -n 19 cargo install starship

RUN echo 'alias ls="eza"' >> /root/.config/fish/config.fish
RUN echo 'starship init fish | source' >> /root/.config/fish/config.fish
RUN /root/.cargo/bin/starship preset nerd-font-symbols -o ~/.config/starship.toml

# bata gef
WORKDIR /root/tools
RUN wget -q https://raw.githubusercontent.com/bata24/gef/dev/install.sh -O- | sh

# ptr command
RUN echo '#!/bin/bash' > /usr/local/bin/ptr
RUN echo 'gdb -q -p $(pidof $1)' >> /usr/local/bin/ptr
RUN chmod +x /usr/local/bin/ptr
RUN echo 'alias ptr="/usr/local/bin/ptr"' >> /root/.config/fish/config.fish

# gdb config
RUN echo 'set follow-fork-mode parent' >> /root/.gdbinit

# ruby tools
RUN gem install seccomp-tools --no-document --force

# glibc tools
WORKDIR /root/
RUN git clone https://github.com/bminor/glibc
RUN ln -s /root/glibc /root/workspace/glibc
RUN git clone https://github.com/matrix1001/glibc-all-in-one
RUN ln -s /root/glibc-all-in-one /root/workspace/glibc-all-in-one

ENV LC_CTYPE C.UTF-8

# bison
RUN wget https://ftp.gnu.org/gnu/bison/bison-3.5.1.tar.gz && \
    tar -xvf bison-3.5.1.tar.gz -C /home/${USERNAME}/ && \
    cd /home/${USERNAME}/bison-3.5.1 && \
    ./configure && make && make install && make clean


RUN echo "set -x LC_CTYPE C.UTF-8" >> /root/.config/fish/config.fish
WORKDIR /root/workspace

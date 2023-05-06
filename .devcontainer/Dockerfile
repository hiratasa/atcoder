# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.148.1/containers/cpp/.devcontainer/base.Dockerfile

FROM mcr.microsoft.com/vscode/devcontainers/cpp:0-ubuntu-20.04

# Boost
ENV BOOST_VERSION 1.72.0
RUN cd /tmp \
    && curl -L https://boostorg.jfrog.io/artifactory/main/release/${BOOST_VERSION}/source/boost_$(echo ${BOOST_VERSION} | sed -e 's/\./_/g').tar.gz | tar xz \
    && cd boost_$(echo ${BOOST_VERSION} | sed -e 's/\./_/g') \
    && ./bootstrap.sh --with-toolset=gcc --without-libraries=mpi,graph_parallel,python \
    && ./b2 -j6 toolset=gcc variant=release link=static runtime-link=static cxxflags="-std=c++17" stage \
    && ./b2 -j6 toolset=gcc variant=release link=static runtime-link=static cxxflags="-std=c++17" --prefix=/usr/local install \
    && cd ../ \
    && rm -rf boost_$(echo ${BOOST_VERSION} | sed -e 's/\./_/g')

# AC-library
RUN cd /tmp \
    && mkdir ac-library \
    && cd ac-library \
    && wget https://img.atcoder.jp/practice2/ac-library.zip \
    && unzip ac-library.zip \
    && cp -r ./atcoder /usr/include/ \
    && cd ../ \
    && rm -rf /tmp/ac-library

# This should be placed after boost install
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends gdb vim

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    # This should be done BEFORE chmod (because this modifies permission.)
    && cargo install cargo-snippet --features="binaries" --locked \
    && chmod a+w -R ${CARGO_HOME} ${RUSTUP_HOME}


RUN mkdir -p /home/vscode/.oh-my-bash/custom \
    && echo "shopt -u dirspell; shopt -u cdspell" > /home/vscode/.oh-my-bash/custom/shopt_ext.sh
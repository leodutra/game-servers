ARG TARGETARCH
ARG TERRARIA_VERSION=1454

# ==========================================
# Stage 1: Build Box64 (ARM64 only)
# ==========================================
FROM debian:bookworm-slim AS build-box64
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y \
    git \
    build-essential \
    cmake \
    python3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /src
RUN git clone https://github.com/ptitseb/box64.git . \
    && mkdir build \
    && cd build \
    && cmake .. -DRPI5ARM64=1 -DCMAKE_BUILD_TYPE=RelWithDebInfo \
    && make -j$(nproc) \
    && make install DESTDIR=/tmp/install

# ==========================================
# Stage 2: Base for ARM64 (Runtime)
# ==========================================
FROM debian:bookworm-slim AS base-arm64
ENV DEBIAN_FRONTEND=noninteractive

# Install Box64 from build stage
COPY --from=build-box64 /tmp/install /

# Install x86_64 libraries required by Terraria (via Box64)
RUN dpkg --add-architecture amd64 \
    && apt-get update && apt-get install -y \
    libc6:amd64 \
    libgcc-s1:amd64 \
    libstdc++6:amd64 \
    libfaudio0 \
    wget \
    unzip \
    gnupg \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# ==========================================
# Stage 3: Base for AMD64 (Native)
# ==========================================
FROM debian:bookworm-slim AS base-amd64
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y \
    wget \
    unzip \
    gnupg \
    ca-certificates \
    libfaudio0 \
    && rm -rf /var/lib/apt/lists/*

# ==========================================
# Final Stage
# ==========================================
FROM base-${TARGETARCH} AS final
ARG TERRARIA_VERSION
ARG DOWNLOAD_URL=https://terraria.org/api/download/pc-dedicated-server/terraria-server-${TERRARIA_VERSION}.zip

WORKDIR /terraria

# Download and Extract Terraria
RUN wget -q ${DOWNLOAD_URL} -O terraria.zip \
    && unzip -q terraria.zip \
    && mv ${TERRARIA_VERSION}/Linux/* . \
    && rm -rf ${TERRARIA_VERSION} terraria.zip \
    && chmod +x TerrariaServer.bin.x86_64

# Copy entrypoint script
COPY entrypoint.sh /terraria/entrypoint.sh
RUN chmod +x /terraria/entrypoint.sh

VOLUME /terraria/Worlds
EXPOSE 7777

ENTRYPOINT ["/terraria/entrypoint.sh"]
CMD ["-config", "serverconfig.txt"]

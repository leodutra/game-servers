FROM debian:bookworm-slim

ARG TARGETARCH
ARG TERRARIA_VERSION=1454
ARG DOWNLOAD_URL=https://terraria.org/api/download/pc-dedicated-server/terraria-server-${TERRARIA_VERSION}.zip

ENV DEBIAN_FRONTEND=noninteractive

# Box64 Environment Variables for RPi 5 (16k page size)
ENV BOX64_PAGE_SIZE=16384
ENV BOX64_DYNAREC_STRONGMEM=1
ENV BOX64_DYNAREC_BIGBLOCK=0
ENV BOX64_DYNAREC_FASTROUND=1


# 1. Install prerequisites and setup the Box64 Repo (ARM64 only)
# We use the 'ryanfortner' repo which provides specific builds for Pi 5
RUN apt-get update && apt-get install -y \
    wget \
    unzip \
    gnupg \
    ca-certificates \
    libfaudio0 \
    && if [ "$TARGETARCH" = "arm64" ]; then \
    wget https://ryanfortner.github.io/box64-debs/box64.list -O /etc/apt/sources.list.d/box64.list \
    && wget -qO- https://ryanfortner.github.io/box64-debs/KEY.gpg | gpg --dearmor -o /etc/apt/trusted.gpg.d/box64.gpg \
    && dpkg --add-architecture amd64 \
    && apt-get update \
    && apt-get install -y box64-rpi5arm64ps16k libc6:amd64 libgcc-s1:amd64 libstdc++6:amd64; \
    fi \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /terraria

# 2. Download and Extract Terraria
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

# 3. Entrypoint
ENTRYPOINT ["/terraria/entrypoint.sh"]
CMD ["-config", "serverconfig.txt"]

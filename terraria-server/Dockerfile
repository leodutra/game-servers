FROM debian:bookworm-slim

ARG TERRARIA_VERSION=1450
ARG DOWNLOAD_URL=https://terraria.org/api/download/pc-dedicated-server/terraria-server-${TERRARIA_VERSION}.zip

ENV DEBIAN_FRONTEND=noninteractive

# 1. Install prerequisites and setup the Box64 Repo
# We use the 'ryanfortner' repo which provides specific builds for Pi 5
RUN apt-get update && apt-get install -y \
    wget \
    unzip \
    gnupg \
    ca-certificates \
    && wget https://ryanfortner.github.io/box64-debs/box64.list -O /etc/apt/sources.list.d/box64.list \
    && wget -qO- https://ryanfortner.github.io/box64-debs/KEY.gpg | gpg --dearmor -o /etc/apt/trusted.gpg.d/box64.gpg \
    && apt-get update \
    && apt-get install -y box64-rpi5arm64 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /terraria

# 2. Download and Extract Terraria
RUN wget -q ${DOWNLOAD_URL} -O terraria.zip \
    && unzip -q terraria.zip \
    && mv ${TERRARIA_VERSION}/Linux/* . \
    && rm -rf ${TERRARIA_VERSION} terraria.zip \
    && chmod +x TerrariaServer.bin.x86_64

VOLUME /terraria/Worlds
EXPOSE 7777

# 3. Entrypoint
ENTRYPOINT ["box64", "./TerrariaServer.bin.x86_64", "-config", "serverconfig.txt"]

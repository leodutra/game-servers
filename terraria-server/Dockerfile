ARG TARGETARCH
ARG TERRARIA_VERSION=1454

# --- Base for AMD64 (Native) ---
FROM debian:bookworm-slim AS base-amd64
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y \
    wget \
    unzip \
    gnupg \
    ca-certificates \
    libfaudio0 \
    && rm -rf /var/lib/apt/lists/*

# --- Base for ARM64 (Box64 on Pi 5) ---
# We use the official Box64 image optimized for Pi 5 (16k pages)
FROM ryanfortner/box64:rpi5 AS base-arm64
ENV DEBIAN_FRONTEND=noninteractive
# Box64 image might be minimal, install tools
RUN apt-get update && apt-get install -y \
    wget \
    unzip \
    gnupg \
    ca-certificates \
    libfaudio0 \
    && rm -rf /var/lib/apt/lists/*

# --- Final Stage ---
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

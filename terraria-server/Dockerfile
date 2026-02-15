# UPDATED: Use Debian Bookworm with Mono (Stable & Supported)
FROM debian:bookworm-slim

# Install Mono and tools
ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y \
    mono-runtime \
    mono-complete \
    unzip \
    wget \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /terraria

# --- DOWNLOAD SECTION ---
ARG TERRARIA_VERSION=1454
ARG DOWNLOAD_URL=https://terraria.org/api/download/pc-dedicated-server/terraria-server-${TERRARIA_VERSION}.zip

RUN wget -q ${DOWNLOAD_URL} -O server.zip \
    && unzip -q server.zip \
    # Move the Linux files
    && cp -r ${TERRARIA_VERSION}/Linux/* . \
    && rm -rf ${TERRARIA_VERSION} server.zip \
    # Remove bundled libraries to use system Mono
    && rm -f System*.dll Mono*.dll mscorlib.dll I18N*.dll

# Permissions
RUN chmod +x TerrariaServer.exe

# Create world directory
RUN mkdir -p /terraria/worlds

VOLUME /terraria/worlds
EXPOSE 7777

# RUN THE SERVER NATIVELY WITH MONO
ENTRYPOINT ["mono", "--server", "--gc=sgen", "-O=all", "TerrariaServer.exe"]
CMD ["-config", "serverconfig.txt"]

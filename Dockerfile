FROM rust:latest

RUN apt-get update && apt-get install -y \
    libwebkit2gtk-4.0-dev \
    libgtk-3-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    npm

WORKDIR /app

COPY . /app

RUN cargo install tauri-cli

RUN npm install

EXPOSE 1420

CMD ["npm", "run", "tauri", "dev"]
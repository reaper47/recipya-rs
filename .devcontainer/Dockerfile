FROM mcr.microsoft.com/devcontainers/rust:latest

RUN apt-get update && apt-get install -y curl hugo
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.0/install.sh | bash
RUN nvm install 20
RUN npm install -g @go-task/cli

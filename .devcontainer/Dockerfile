FROM mcr.microsoft.com/devcontainers/rust:latest

RUN apt-get update && apt-get install -y curl hugo && apt-get -y install --no-install-recommends postgresql-client \
&& apt-get clean -y && rm -rf /var/lib/apt/lists/*

ENV NVM_DIR /usr/local/nvm
RUN mkdir -p $NVM_DIR
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.0/install.sh | bash 
ENV NODE_VERSION v22.6.0
RUN /bin/bash -c "source $NVM_DIR/nvm.sh && nvm install 22 && nvm use --delete-prefix 22"
ENV NODE_PATH $NVM_DIR/versions/node/$NODE_VERSION/lib/node_modules
ENV PATH      $NVM_DIR/versions/node/$NODE_VERSION/bin:$PATH

RUN npm install -g @go-task/cli
RUN cargo install diesel_cli

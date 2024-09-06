FROM rust:1.80.1-slim-bullseye
ARG UID
ARG GID
ARG USER
RUN groupadd -g ${GID} ${USER} \
 && useradd -u ${UID} -g ${GID} -m ${USER} \
 && mkdir -p /home/${USER}/.ssh \
 && chown -R ${USER}:${USER} /home/${USER}
RUN apt-get -y update \
 && apt-get -y --no-install-recommends install \
    git \
    ssh \
 && apt-get clean \
 && rm -rf /var/lib/apt/lists/*
USER ${USER}
RUN ssh-keyscan github.com >> /home/${USER}/.ssh/known_hosts
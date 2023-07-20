FROM nginx AS base

FROM base AS production-amd64
ENV path "./release/dashboard-static"

FROM base as production-arm64
ENV path "./release/dashboard-static"

ARG TARGETARCH
FROM production-$TARGETARCH AS production
ARG lodestone_version

WORKDIR /usr/share/nginx/html

COPY $path/out/ ./

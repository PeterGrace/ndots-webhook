FROM dreg.vsix.me:9443/keim-rust-sccache:latest AS build
ARG SCCACHE_BUCKET
ARG SCCACHE_ENDPOINT
ARG SCCACHE_S3_USE_SSL
ARG AWS_ACCESS_KEY_ID
ARG AWS_SECRET_ACCESS_KEY
USER 0
ADD . /home/app
RUN sccache -s \
 && cargo build --release \
 && sccache -s

FROM dreg.vsix.me:9443/keim-rust-sccache:latest
# Add Tini
USER 0
ENV TINI_VERSION v0.19.0
ADD https://github.com/krallin/tini/releases/download/${TINI_VERSION}/tini /tini
RUN chmod +x /tini
ENTRYPOINT ["/tini", "--"]

# add vim goldangit
RUN apt-get -y update \
 && apt-get -y install vim

# add the app
RUN mkdir /ndots-webhook
RUN chown 1000:1000 /ndots-webhook
USER 1000
WORKDIR /ndots-webhook
COPY --from=build /home/app/target/release/ndots-webhook /ndots-webhook/ndots-webhook
COPY Rocket.toml /ndots-webhook/Rocket.toml
COPY entrypoint.sh /ndots-webhook/entrypoint.sh

CMD ["/ndots-webhook/entrypoint.sh"]

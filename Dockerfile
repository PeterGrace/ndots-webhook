FROM dreg.vsix.me:9443/keim-rust-sccache:latest AS build

ADD . /home/app
RUN cargo build --release

FROM dreg.vsix.me:9443/keim-rust-sccache:latest

COPY --from=build /home/app/target/release/ndots-webhook /ndots-webhook


#!/bin/bash
image=${1:-"foo"}
docker run -ti --rm \
	-v `pwd`/cert.pem:/ndots-webhook/cert.pem \
	-v `pwd`/key.pem:/ndots-webhook/key.pem \
	-p 8443:8443 \
  ${image}

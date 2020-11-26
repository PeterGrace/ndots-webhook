#!/bin/bash
export SCCACHE_BUCKET=sccache
export SCCACHE_ENDPOINT=s3.vsix.me:9000
export SCCACHE_S3_USE_SSL=true

image=${1:-"foo"}

eval $(for envvar in $(vault kv get -format=json kv/minio|jq -r '.data.data|to_entries|map("\(.key)=\(.value|tostring)")|.[]'); do echo export $envvar; done)

docker build -t ${image} \
	--build-arg SCCACHE_BUCKET=${SCCACHE_BUCKET} \
	--build-arg SCCACHE_ENDPOINT=${SCCACHE_ENDPOINT} \
	--build-arg SCCACHE_S3_USE_SSL=${SCCACHE_S3_USE_SSL} \
	--build-arg AWS_ACCESS_KEY_ID=${AWS_ACCESS_KEY_ID} \
	--build-arg AWS_SECRET_ACCESS_KEY=${AWS_SECRET_ACCESS_KEY} \
	.

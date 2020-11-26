#!/bin/bash
export PATH="$PATH:/ndots-webhook"

run_app() {
  ndots-webhook
}

validate_prod() {
	if [[ -z "SSL_PATH" ]]
	then
		echo "you must specify the path to cert.pem and key.pem for production mode."
		exit 1
	fi
	export ROCKET_GLOBAL_TLS_CERTS="${SSL_PATH}/cert.pem"
	export ROCKET_GLOBAL_TLS_KEY="${SSL_PATH}/key.pem"
}

if [[ -z "$ROCKET_ENV" ]]
then
	echo "ROCKET_ENV isn't set.  Assuming development settings."
	export ROCKET_ENV=development
fi

case $ROCKET_ENV in
	"development")
	  run_app
	  ;;
	"production")
	  validate_prod
	  run_app
	;;
	"*")
	;;
esac

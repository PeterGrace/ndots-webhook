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
	export ROCKET_TLS="{certs=\"${SSL_PATH}/cert.pem\",key=\"${SSL_PATH}/key.pem\"}"

}

if [[ ! -z "$DEBUG" ]]
then
	echo "debug requested, sleeping 86400"
	/bin/sleep 86400
	exit 1
fi


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

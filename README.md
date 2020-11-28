# ndots-webhook
[![Build Status](https://drone.k.vsix.me/api/badges/support/ndots-webhook/status.svg?ref=refs/heads/main)](https://drone.k.vsix.me/support/ndots-webhook)
I'm working on a MutatingWebhook for Kubernetes that will allow me to alter DNSOptions `ndots` for any pod in the system based on annotation.

The reason why I'm writing this is because helm charts and operators often don't include dnsoptions in their specifications, but they generally
allow the pass-through of annotations into pods, which will allow me to dynamically change the pod spec via MutatingWebhook to facilitate this fix.


## How to install
The files in the repo assume you're installing the service to ndots-webhook, namespace ndots-webhook.  If you don't want this, you'll need to edit the files in deploy/ to coincide with the changes.  No helm charts here yet, but I may create some in the future.

  1. Clone this repo.
  1. Create a new webhook tls key pair.
```
service=<my-service-name> namespace=<my-namespace-name> secret=<what-to-name-secret> ./deploy/webhook-create-signed-cert.sh
e.g.
service=ndots-webhook namespace=ndots-webhook secret=ndots-webhook ./deploy/webhook-create-signed-cert.sh
```
  1. Copy the cert.pem payload out of that secret and put it into ./deploy/mutating_webhook.yaml under caBundle.
  1. `kubectl apply -k ` in the deploy/ folder
NOTE: I use pod security policies in my kubernetes cluster.  The psp object might not apply.  Just delete it if you don't need it.

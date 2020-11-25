# ndots-webhook
[![Build Status](https://drone.k.vsix.me/api/badges/support/ndots-webhook/status.svg?ref=refs/heads/main)](https://drone.k.vsix.me/support/ndots-webhook)
I'm working on a MutatingWebhook for Kubernetes that will allow me to alter DNSOptions `ndots` for any pod in the system based on annotation.

The reason why I'm writing this is because helm charts and operators often don't include dnsoptions in their specifications, but they generally
allow the pass-through of annotations into pods, which will allow me to dynamically change the pod spec via MutatingWebhook to facilitate this fix.

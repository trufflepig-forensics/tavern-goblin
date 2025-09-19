#!/usr/bin/env bash

CA_KEY="/tmp/tavern-goblin-ca.key"
CA_CRT="data/ca-certificates/ca-dev.crt"

NGINX_KEY="data/conf/nginx/tavern-goblin-dev.key"
NGINX_CRT="data/conf/nginx/tavern-goblin-dev.crt"

openssl \
    req -new -x509 \
    -newkey rsa:4096 \
    -keyout $CA_KEY \
    -noenc \
    -out $CA_CRT \
    -subj '/C=DE/ST=Bavaria/L=Wolnzach/O=Trufflepig Forensics/OU=Software Engineering/CN=Tavern Goblin CA' \
    -addext 'subjectKeyIdentifier = hash' \
    -addext 'basicConstraints = critical,CA:true' \
    -addext 'keyUsage = critical, digitalSignature, cRLSign, keyCertSign' \
    -not_after 99991231235959Z

openssl \
    req -new -x509 \
    -newkey rsa:4096 \
    -keyout $NGINX_KEY \
    -noenc \
    -out $NGINX_CRT \
    -CA $CA_CRT \
    -CAkey  $CA_KEY \
    -subj '/C=DE/ST=Bavaria/L=Wolnzach/O=Trufflepig Forensics/OU=Software Engineering/CN=Tavern Goblin Server' \
    -addext 'basicConstraints = critical,CA:false' \
    -addext 'subjectAltName = DNS:localhost, DNS:nginx-dev' \
    -not_after 99991231235959Z
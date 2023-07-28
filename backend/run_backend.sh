podman build --tag tstats:pre-alpha -f Dockerfile
podman run --rm -p 3000:3000 --net tstats --name tstats-backend tstats:pre-alpha

podman build --tag tstats-backend-manual:pre-alpha -f Dockerfile
podman run --rm -p 3000:3000 --net tstats --name tstats-backend-manual tstats-backend-manual:pre-alpha

# Actix + mongo example

To run mongo container:
```bash
podman run --name mongo_example -d -p 27017:27017 -e MONGO_INITDB_ROOT_USERNAME=user -e MONGO_INITDB_ROOT_PASSWORD=pass mongo:latest
```
To run this in docker:

docker build -t blocktime-docker .
docker run --rm -it --init -p 12345:12345 blocktime-docker
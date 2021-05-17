To run this in docker:

docker build -t blocktime-docker .
docker run --rm -it --init -p 12345:12345 blocktime-docker

Note:
I'm not using etherscan.io api key so queries are limited to 1/5sec. One timestamp read needs 2 queries so it's beeing updated every ~10 sec.
export RUST_LOG=debug
export CONFIG_FILE_PATH=example-website/config.yaml
export CONFIG_FILE_GITHUB_PROJECT=ryanlaycock/gitsite

GIT_SHA=$(shell git rev-parse --short=7 HEAD)

run:
	cargo run

git-sha:
	echo $(GIT_SHA)

docker-build-push:
	docker build . -t ryanlaycock/gitsite:$(GIT_SHA) -f Dockerfile
	docker push ryanlaycock/gitsite:$(GIT_SHA)
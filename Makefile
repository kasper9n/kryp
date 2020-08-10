server-sh:
	docker run --rm -v "$(PWD)"/server:/go/src/app -w /go/src/app -it golang:1.14-alpine /bin/sh

FROM golang:1.14-alpine AS dev-stage
WORKDIR /go/src/app
RUN apk add git mercurial \
  && go get github.com/canthefason/go-watcher \
  && go install github.com/canthefason/go-watcher/cmd/watcher
COPY go.mod go.sum ./
RUN go mod download
COPY . .
RUN go build -o /goapp
RUN chmod +x /goapp
ENTRYPOINT watcher -i -o /goapp

FROM scratch AS bin
COPY --from=dev-stage /goapp /
# needs to be array (not shell form) because scratch image doens't have shell
ENTRYPOINT ["/goapp"]

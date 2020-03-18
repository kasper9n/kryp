FROM golang:1.14-alpine AS dev-stage
WORKDIR /go/src/app
RUN apk add git mercurial \
  && go get github.com/githubnemo/CompileDaemon
COPY go.mod go.sum ./
RUN go mod download
COPY . .
RUN go build
ENTRYPOINT CompileDaemon -command=./cryptrack-api -log-prefix=false

FROM alpine
WORKDIR /go/src/app
COPY --from=dev-stage /go/src/app/cryptrack-api .
ENTRYPOINT ./cryptrack-api

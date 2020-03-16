FROM golang:1.14-alpine AS api-dev-stage
WORKDIR /go/src/app
RUN apk add git mercurial \
  && go get github.com/githubnemo/CompileDaemon
COPY go.mod go.sum ./
RUN go mod download
COPY . .
RUN go build
ENTRYPOINT CompileDaemon -command=./cryptrack-api -log-prefix=false

FROM golang:1.14-alpine
COPY --from=api-dev-stage /go/src/app /go/src/app
WORKDIR /go/src/app

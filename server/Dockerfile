FROM golang:1.14-alpine AS dev-stage
WORKDIR /go/src/app
RUN apk add git mercurial \
  && go get github.com/githubnemo/CompileDaemon
# RUN go mod init krypserver
# COPY go.mod go.sum ./
# RUN go mod download
COPY . .
RUN go build -o /goapp
RUN chmod +x /goapp
# RUN ./goapp
ENTRYPOINT CompileDaemon -command="/goapp" -log-prefix=false -build="go build -o /goapp"

FROM scratch AS bin
COPY --from=dev-stage /goapp /
# needs to be array (not shell form) because scratch image doens't have shell
ENTRYPOINT ["/goapp"]

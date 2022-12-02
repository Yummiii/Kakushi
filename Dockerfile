FROM alpine:3.17
WORKDIR /app
RUN apk update
RUN apk add wget
RUN wget https://github.com/Yummiii/Kakushi/releases/download/latest/kakushi.$(uname -m)-musl
RUN chmod +x kakushi.$(uname -m)-musl
ENTRYPOINT exec kakushi.$(uname -m)-musl
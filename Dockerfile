FROM alpine:3.17
WORKDIR /app
RUN apk update
RUN apk add wget
RUN echo $(uname -m)
RUN wget https://github.com/Yummiii/Kakushi/releases/download/latest/kakushi.$(uname -m)-musl -O /app/kakushi
RUN chmod +x ./kakushi
ENTRYPOINT [ "./kakushi" ]

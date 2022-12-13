FROM debian:latest
WORKDIR /app
RUN apt update && apt upgrade -y
RUN apt install wget -y
RUN echo $(uname -m)
RUN wget https://github.com/Yummiii/Kakushi/releases/download/latest/kakushi.$(uname -m) -O /app/kakushi
RUN chmod +x /app/kakushi
ENTRYPOINT [ "./app/kakushi" ]

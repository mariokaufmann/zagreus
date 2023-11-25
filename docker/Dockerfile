FROM gcr.io/distroless/cc-debian12

ENV HOME="/home"

EXPOSE 58180

COPY . /app
WORKDIR /app

ENTRYPOINT ["./zagreus-server"]
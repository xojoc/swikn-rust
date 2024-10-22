FROM rust:1.80

WORKDIR /usr/src/myapp
COPY . .

# RUN cargo install --path .
RUN make dist

EXPOSE 3000

CMD ["myapp"]


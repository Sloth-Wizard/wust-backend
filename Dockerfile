FROM rust:1.59 as build

# create a new empty shell project
RUN USER=root cargo new --bin wust
WORKDIR /wust

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# copy the env file
# change this to the one you want to use when using docker
COPY ./.env.dev ./.env

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/wust*
RUN cargo build --release

# our final base
FROM rust:1.59-slim-buster

# copy the build artifact from the build stage
COPY --from=build /wust/target/release/wust .

# set the startup command to run your binary
CMD ["./wust"]

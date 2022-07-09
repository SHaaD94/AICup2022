FROM rust
RUN apt-get update && apt-get install -y jq unzip

ENV MOUNT_POINT="/opt/mount-point"
ENV SOLUTION_CODE_PATH="/opt/client/solution"
COPY . $SOLUTION_CODE_PATH
WORKDIR $SOLUTION_CODE_PATH
CMD ["bash", "entrypoint.sh"]

ENV CARGO_TARGET_DIR=/opt/cargo-target
RUN cargo build --release
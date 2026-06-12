# Minimal image so the example proves `docker build` runs in CI via the SAME
# `mise run ci` — on the Linux matrix cell it builds; on macOS/Windows runners
# (no docker daemon) `docker:build` skips. Build-only, never pushed.
FROM alpine:3.20
RUN echo "hello from .github-example" > /hello.txt
CMD ["cat", "/hello.txt"]

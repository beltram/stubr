source ./bench/wrk.sh

wiremock_test() {
  path=$1
  duration=$2
  vu=$3
  /usr/local/bin/wiremock \
    --no-request-journal \
    --disable-banner \
    --disable-gzip \
    --port 8001 \
    --root-dir bench &
  PID=$!
  sleep 5
  scenario="| wiremock-${path} (${duration}s / ${vu}) | "
  uri="http://localhost:8001/${path}"
  wrk_test "$uri" "$scenario" "$duration" "$vu"
  kill "$PID"
}

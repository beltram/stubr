source ./bench/wrk.sh

wiremock_bench() {
  stubs=$1
  path=$2
  duration=$3
  vu=$4
  scenario_name=$5
  echo "Begin scenario '$scenario_name' for $duration s for Wiremock..."
  /usr/local/bin/wiremock \
    --no-request-journal \
    --disable-banner \
    --disable-gzip \
    --disable-request-logging \
    --global-response-templating \
    --async-response-enabled "true" \
    --async-response-threads "20" \
    --port 8081 \
    --root-dir "$stubs" &>/dev/null &
  wiremock_pid=$!
  sleep 5
  scenario="| wiremock-${path} (${duration}s / ${vu}) | "
  uri="http://localhost:8081/${path}"
  wrk_test "$uri" "$scenario" "$duration" "$vu" "$wiremock_pid" false
  kill "$wiremock_pid" &>/dev/null
  echo "...end scenario '$scenario_name' for Wiremock"
}

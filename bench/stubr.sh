source ./bench/wrk.sh

stubr_test() {
  path=$1
  duration=$2
  vu=$3
  cargo build -q --release
  cargo run -q --release -- bench/mappings &
  PID=$!
  sleep 5
  scenario="| stubr-${path} (${duration}s / ${vu}) | "
  uri="http://localhost:8000/${path}"
  wrk_test "$uri" "$scenario" "$duration" "$vu"
  kill "$PID"
}

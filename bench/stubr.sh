source ./bench/wrk.sh

stubr_bench() {
  path=$1
  duration=$2
  vu=$3
  tmp=bench/stubr-out-tmp.txt
  cargo build -q --release
  cargo run -q --release -- bench/mappings -p 10000 &>$tmp &
  sleep 2
  addr="http://localhost:10000"
  PID=$!
  sleep 2
  scenario="| stubr-${path} (${duration}s / ${vu}) | "
  uri="${addr}/${path}"
  wrk_test "$uri" "$scenario" "$duration" "$vu" "$PID"
  kill "$PID"
  rm $tmp
}

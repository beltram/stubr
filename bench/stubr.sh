source ./bench/wrk.sh

stubr_bench() {
  path=$1
  duration=$2
  vu=$3
  tmp=bench/stubr-out-tmp.txt
  cargo build -q --release
  cargo run -q --release -- bench/mappings &>$tmp &
  sleep 2
  addr="$(cat $tmp | grep 'stubr' | sed 's/.*stubr server on //')"
  PID=$!
  sleep 2
  scenario="| stubr-${path} (${duration}s / ${vu}) | "
  uri="${addr}/${path}"
  wrk_test "$uri" "$scenario" "$duration" "$vu" "$PID"
  kill "$PID"
  rm $tmp
}

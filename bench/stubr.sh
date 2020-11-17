source ./bench/wrk.sh

stubr_test() {
  path=$1
  duration=$2
  vu=$3
  tmp=bench/stubr-out-tmp.txt
  cargo build -q --release
  cargo run -q --release -- bench/mappings &>$tmp &
  sleep 2
  addr="$(cat $tmp | grep 'stubr' | sed 's/.*stubr server on //')"
  PID=$!
  sleep 5
  scenario="| stubr-${path} (${duration}s / ${vu}) | "
  uri="${addr}/${path}"
  wrk_test "$uri" "$scenario" "$duration" "$vu"
  kill "$PID"
  rm $tmp
}

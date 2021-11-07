source ./bench/wrk.sh

stubr_bench() {
  stubs=$1
  path=$2
  duration=$3
  vu=$4
  scenario_name=$5
  echo "Begin scenario '$scenario_name' for $duration s for stubr..."
  tmp=bench/stubr-out-tmp.txt
  cargo build -q --release 2>/dev/null
  cargo run -q --release --bin stubr -- --root-dir "$stubs" &>$tmp &
  sleep 5
  chmod 777 $tmp
  log=$(cat "$tmp" | grep "Started stubr in ")
  port=${log#*127.0.0.1:}
  addr="http://127.0.0.1:$port"
  stubr_pid=$!
  sleep 1
  scenario="| stubr-${path} (${duration}s / ${vu}) | "
  uri="${addr}/${path}"
  wrk_test "$uri" "$scenario" "$duration" "$vu" "$stubr_pid" true
  kill "$stubr_pid" &>/dev/null
  rm $tmp
  echo "...end scenario '$scenario_name' for stubr"
}

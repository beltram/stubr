source ./bench/stubr.sh
source ./bench/wiremock.sh

scenario() {
  stubs=$1
  path=$2
  duration=$3
  vu=$4
  scenario_name=$5
  echo "Begin scenario '$scenario_name' ..."
  echo "
  ### $scenario_name
| scenario (duration / vu) | avg latency (+/-) | avg req/sec (+/-) | total req | total bytes | avg cpu % | avg mem |
|:------------------------:|:-----------------:|:-----------------:|:---------:|:-----------:|:---------:|:-------:|" >>$OUTPUT
  stubr_bench "$stubs" "$path" "$duration" "$vu" "$scenario_name"
  wiremock_bench "$stubs" "$path" "$duration" "$vu" "$scenario_name"
  echo "...end scenario '$scenario_name'"
}

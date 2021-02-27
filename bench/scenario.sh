source ./bench/stubr.sh
source ./bench/wiremock.sh

scenario() {
  path=$1
  duration=$2
  echo "
|  scenario (duration / users) | avg latency (+/-) | avg req/sec (+/-) | total req | total bytes | avg cpu | avg mem(stubr Kb/wiremock Mb) |
|:----------------------------:|:-----------------:|:-----------------:|:---------:|:-----------:|:-------:|:-----------------------------:|" >>$OUTPUT
  stubr_bench "$path" "$duration" 1
  wiremock_bench "$path" "$duration" 1
  stubr_bench "$path" "$duration" 50
  wiremock_bench "$path" "$duration" 50
  stubr_bench "$path" "$duration" 200
  wiremock_bench "$path" "$duration" 200
  stubr_bench "$path" "$duration" 500
  wiremock_bench "$path" "$duration" 500
}

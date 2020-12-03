source ./bench/stubr.sh
source ./bench/wiremock.sh

scenario() {
  path=$1
  duration=$2
  echo "
|  scenario (duration / users) | avg latency (+/-) | avg req/sec (+/-) | total req | total bytes | avg cpu | avg mem(stubr Kb/wiremock Mb) |
|:----------------------------:|:-----------------:|:-----------------:|:---------:|:-----------:|:-------:|:-----------------------------:|" >>$OUTPUT
  stubr_bench "$path" $duration 10
  wiremock_bench "$path" $duration 10
  stubr_bench "$path" $duration 100
  wiremock_bench "$path" $duration 100
  stubr_bench "$path" $duration 200
  wiremock_bench "$path" $duration 200
}

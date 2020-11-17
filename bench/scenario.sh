source ./bench/stubr.sh
source ./bench/wiremock.sh

scenario() {
  path=$1
  echo "
|  scenario (duration / users) | avg latency (+/-) | avg req/sec (+/-) | total req | total bytes |
|:--------------------------:|:-----------------:|:-----------------:|:---------:|:-----------:|" >>$OUTPUT
  stubr_test "$path" 60 1
  wiremock_test "$path" 60 1
  stubr_test "$path" 60 10
  wiremock_test "$path" 60 10
  stubr_test "$path" 60 100
  wiremock_test "$path" 60 100
  stubr_test "$path" 60 200
  wiremock_test "$path" 60 200
}
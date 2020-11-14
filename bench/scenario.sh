source ./bench/stubr.sh
source ./bench/wiremock.sh

scenario() {
  path=$1
  echo "
|  scenario (duration / users) | avg latency (+/-) | avg req/sec (+/-) | total req | total bytes |
|:--------------------------:|:-----------------:|:-----------------:|:---------:|:-----------:|" >>$OUTPUT
  stubr_test "$path" 10 1
  stubr_test "$path" 10 10
  stubr_test "$path" 10 100
  wiremock_test "$path" 10 1
  wiremock_test "$path" 10 10
  wiremock_test "$path" 10 100
}

#echo "BEGIN Wiremock bench"
#echo ""
#/usr/local/bin/wiremock \
#  --no-request-journal \
#  --disable-banner \
#  --disable-gzip \
#  --port 8001 \
#  --root-dir bench &
#WIREMOCK_PID=$!
#sleep 5
#wrk -d $DURATION -c $VU -t $VU http://localhost:8001
#kill "$WIREMOCK_PID"

wrk_test() {
  uri=$1
  name=$2
  duration=$3
  vu=$4
  wrk -s bench/report.lua -d $duration -c $vu -t $vu $uri \
    | sed -n '/___/,$p' | sed 's/.*___//' \
    | awk -v NAME="$name" '{print NAME $0}' \
    | tail -n 1 >>$OUTPUT
}
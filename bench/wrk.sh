wrk_test() {
  uri=$1
  name=$2
  duration=$3
  vu=$4
  pid=$5
  TOP_OUT=bench/top-out.txt
  top -pid "$pid" -c e -stats cpu,mem &>$TOP_OUT &
  TOP_PID=$!
  WRK_OUT=$(
    wrk -s bench/report.lua -d $duration -c $vu -t $vu $uri |
      sed -n '/___/,$p' | sed 's/.*___//' |
      awk -v NAME="$name" '{print NAME $0}' |
      tail -n 1
  )
  STATS=$(
    cat $TOP_OUT |
      grep -A 1 "%CPU" |
      grep -v "%CPU" |
      grep -v '^--' |
      sed -e 1d |
      awk -F' ' '{ cpu+=$1 ; mem+=$2 } END { print cpu/NR " | " mem/NR " | " }'
  )
  echo "${WRK_OUT}${STATS}" >>$OUTPUT
  rm $TOP_OUT
  kill "$TOP_PID"
}

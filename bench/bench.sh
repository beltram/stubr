source ./bench/scenario.sh
# wrk options
export DURATION=1
export VU=10

# metadata
export OUTPUT=bench/report.md

# print statistics
echo "# Benchmark
* stubr
  * rust version: $(rustc --version | sed 's/.*rustc //' | sed 's/(.*//')
* wiremock
  * java version: $(java --version | grep openjdk)
  * wiremock version: $(readlink /usr/local/bin/wiremock | sed 's/.*standalone\///' | sed 's/\/bin.*//')
" > $OUTPUT

scenario "ping"
source ./bench/scenario.sh
# metadata
export OUTPUT=bench/report.md

# print statistics
echo "# Benchmark
* stubr
  * rust version: $(rustc --version | sed 's/.*rustc //' | sed 's/(.*//')
* wiremock
  * java version: $(java --version | grep openjdk)
  * wiremock version: $(readlink /usr/local/bin/wiremock | sed 's/.*standalone\///' | sed 's/\/bin.*//')
" >$OUTPUT

scenario "ping" 60
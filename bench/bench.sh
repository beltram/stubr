source ./bench/scenario.sh
source ./bench/stub-provider.sh
# metadata
export OUTPUT=bench/README.md


# print statistics
echo "# Benchmark
* stubr
  * version: $(cargo run -q --bin stubr -- -V | sed 's/.*stubr //' | sed 's/(.*//')
  * rust version: $(rustc --version | sed 's/.*rustc //' | sed 's/(.*//')
* wiremock
  * java version: $(java --version | grep openjdk)
  * wiremock version: $(readlink /usr/local/bin/wiremock | sed 's/.*standalone\///' | sed 's/\/bin.*//')
" >$OUTPUT

scenario "bench" "ping" 60 1 "1 vu"
scenario "bench" "ping" 60 100 "100 vu"
scenario "bench" "response-template" 60 100 "response templating"
createManyStubs
scenario "bench/mappings/many" "many" 60 100 "many loaded stubs"
deleteManyStubs
scenario "bench" "ping" 2 10 "cold start"
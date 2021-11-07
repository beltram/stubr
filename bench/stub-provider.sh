createManyStubs() {
  for i in {1..100}
  do
     echo "{\"request\": {\"urlPath\": \"/$i\"}, \"response\": {}}" > "bench/mappings/many/mappings/$i.json"
  done

  echo "{\"request\": {\"urlPath\": \"/many\"}, \"response\": {}}" > bench/mappings/many/mappings/many.json
}
deleteManyStubs() {
  rm -rf bench/mappings/many/mappings/*.json
}
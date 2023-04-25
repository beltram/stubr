# gRPC

Stubr also supports mocking a gRPC service ! To do so, it
leverages [Protobuf to json mapping](https://protobuf.dev/programming-guides/proto3/#json) in order to ; first to reuse
all the request matchers and response templates already available and implemented ; then, to let you instantiate (give a
value to the fields) your messages using json (you can't assign a value to a field in Protobuf).  

The API looks like this:

```json
{
  "protoFile": "path/to/grpc.proto", // protobuf file where gRPC service & protobuf messages are defined
  "grpcRequest": {
    "message": "Pet", // name of the body's message in 'protoFile' 
    "service": "PetStore", // (optional) name of the gRPC service to mock, supports Regex
    "method": "createDog", // (optional) name of the gRPC method to mock, supports Regex
    "bodyPatterns": [
      {
        "equalToJson": { // literally the same matchers as in http
          "name": "Rex",
          "race": "dog"
        }
      }
    ]
  },
  "grpcResponse": {
    "status": "OK", // or "CANCELLED", "NOT_FOUND" etc..
    "message": "Pet", // name of the body's message in 'protoFile'
    "body": { // literally the same as in http, supports templating too
      "id": 1234,
      "name": "{{jsonPath request.body '$.name'}}",
      "race": "{{jsonPath request.body '$.race'}}",
      "action": "{{request.method}}", // only 2 differences with standard templates
      "service": "{{request.service}}"
    },
    "transformers": [ // required for response templating
      "response-template"
    ]
  }
}
```

@0xf6bb5728caebd6c9;

interface HelloWorld {
  struct HelloRequest {
    name @0 :Text;
  }
  
  struct HelloReply {
    message @0 :Text;
  }

  sayHello @0 (request: HelloRequest) -> (reply: HelloReply);

}
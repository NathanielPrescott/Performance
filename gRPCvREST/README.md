## Idea
This subproject is built for testing gRPC calls against REST calls using WebAssembly.
There are many factors that are involved in the building of this performance test.
Primarily the goal of this project is to experiment with WebAssembly and create a template for testing of gRPC 
communication from the web browser to a service allowing for the front end to be completely ignorant to the backend services.
The idea I had was to allow for backend developers after creating a service to then create a paired service hook in the
WebAssembly project using gRPC.  This would give increased performance and security to a companies services.

## Tests
Currently, the service project has a text string and an image that can be requested.  There is only one implementation of
the text string and the image has 4 implementations (Small, Medium, Large, and Original).  The image is provided under the 
Service/src/files directory.  The Client project runs through the REST services first and then the gRPC WASM services.
Be mindful that due to the exponential increase in time for the conversion of data between the WASM and JS layers, that
the Original Image calls will take a long time to complete.

## Learning
Some things I found out during my exploration of this idea:
1. WebAssembly can be easily read after being compiled (not encrypted).
   1. This eliminates the security aspect of my original idea.
2. WebAssembly currently does not have any direct web access and any calls it makes will be converted to a JavaScript call.
   1. This eliminates the performance aspect of my original idea.  Though this may change in the future with the current discussions and timeline priorities.

## Structure
Currently, there are technically 4 projects here that I will list below:
1. Client
   1. Service
2. Service
3. Test_Service

### Client
This is a JavaScript project that implements a REST call to the service project.
This project also calls a WebAssembly project that implements a gRPC call to the service project.

### Client -> Service
This is a Rust WebAssembly project that implements a gRPC call to the service project.
This project is called from the Client project.

### Service
This is a Rust project that handles REST and gRPC calls.
This service needs to be running for any of the other projects to work.

### Test Service
This is a Rust project that handles gRPC calls to verify the performance of what a service to service call should look like.

## Performance
Currently, due to the data conversion that needs to occur between WebAssembly and the JS layer, the performance of gRPC with
WASM is exponentially slower which makes sense now that I have learned the limits of WebAssembly's data transport layer.
The performance using the test_service project makes more sense using gRPC directly between two services, especially after I 
added streaming to the gRPC proto.

I'll post images of my performance tests here later.

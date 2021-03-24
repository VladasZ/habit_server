
#include "Log.hpp"
#include "LoginRequest.hpp"
#include "HelloRequest.hpp"
#include "NotFoundRequest.hpp"
#include "RegistrationRequest.hpp"
#include "HabitRequestHandler.hpp"


using namespace std;
using namespace habit;
using namespace Poco::Net;

#define ROUTE(RequestType) \
if (uri == RequestType::url()) {\
    return new RequestType;\
}

HTTPRequestHandler* HabitRequestHandler::createRequestHandler(const HTTPServerRequest& request) {
    auto uri = request.getURI();

    Log << uri;

    ROUTE(HelloRequest);
    ROUTE(LoginRequest);
    ROUTE(RegistrationRequest);

    return new NotFoundRequest;
}


#include "HelloRequest.hpp"

using namespace std;
using namespace habit;
using namespace mapping;
using namespace Poco::Net;

void HelloRequest::handle() {
    send() << "Hello! Ti tigigig tigidignaya, tigidig tigidig tigidi4ka!";
}

string HelloRequest::name() const {
    return "Hello request";
}

string HelloRequest::url() {
    return "/hello";
}

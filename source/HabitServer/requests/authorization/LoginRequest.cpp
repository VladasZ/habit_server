
#include "LoginRequest.hpp"
#include "TokenStorage.hpp"
#include "HabitDatabase.hpp"

using namespace std;
using namespace habit;
using namespace mapping;
using namespace Poco::Net;

void LoginRequest::handle() {

    auto local_user = database().select_first_where<&User::email>(request_object.login);

    if (local_user.not_found()) {
        respond_error("User " + request_object.login + " does not exist");
        return;
    }

    //request_object.hash_password();

    User user = local_user;

    if (request_object.password != user.password) {
        respond_error("Invalid password");
        return;
    }

    send() << JSON {{ "token", TokenStorage<User>::generate_and_store(user) }};
}

string LoginRequest::name() const {
    return "Login request";
}

vector<string> LoginRequest::required_json_keys() const {
    return {
            "login",
            "password"
    };
}

string LoginRequest::url() {
    return "/login";
}

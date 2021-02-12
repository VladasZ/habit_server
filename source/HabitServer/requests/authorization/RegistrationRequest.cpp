
#include "Log.hpp"
#include "TokenStorage.hpp"
#include "HabitDatabase.hpp"
#include "RegistrationRequest.hpp"

using namespace std;
using namespace habit;
using namespace mapping;
using namespace Poco::Net;


void RegistrationRequest::handle() {

    if (database().select_first_where<&User::email>(request_object.login).found()) {
        set_status(HTTPServerResponse::HTTPStatus::HTTP_NOT_ACCEPTABLE);
        std::string message = "User with email " + request_object.login + " already exists.";
        respond_error("Invalid input: "+ message);
        return;
    }

    User new_user;
    new_user.email = request_object.login;
    new_user.password = request_object.password;

    //  new_user.hash_password();

    database().insert(new_user);

    auto local_user = database().select_first_where<&User::email>(new_user.email);

    if (local_user.not_found()) {
        respond_error("Server error. Please show this error to server developer. User not found after insert");
        return;
    }

    send() <<
           JSON {
                   { "token"  , TokenStorage<User>::generate_and_store(local_user)                  },
                   { "message", "Registration of user " + request_object.login + " was successful." },
           }.dump();
}

std::string RegistrationRequest::name() const {
    return "Registration request";
}

std::vector<std::string> RegistrationRequest::required_json_keys() const {
    return {
            "login",
            "password"
    };
}

std::string RegistrationRequest::url() {
    return "/register";
}


#pragma once

#include <string>

namespace habit {

    class User {

    public:

        int id = -1;

        std::string email;
        std::string password;

        std::string name;
        std::string surname;

        std::string generate_token() const;

    };

};


#pragma once

#include "User.hpp"
#include "HabitBaseRequest.hpp"

namespace habit {

    class LoginRequest : public HabitBaseRequest<LoginData> {
    public:
        void handle() override;
    private:
        std::string name() const override;
        std::vector<std::string> required_json_keys() const override;
    public:
        static std::string url();
    };

}
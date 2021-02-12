
#pragma once

#include "LoginData.hpp"
#include "HabitBaseRequest.hpp"


namespace habit {

    class RegistrationRequest : public HabitBaseRequest<LoginData> {
    public:
        void handle() override;
    private:
        std::string name() const override;
        std::vector<std::string> required_json_keys() const override;
    public:
        static std::string url();
    };

}

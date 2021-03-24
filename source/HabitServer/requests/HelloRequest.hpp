
#pragma once

#include "HabitBaseRequest.hpp"

namespace habit {

    class HelloRequest : public HabitBaseRequest<> {
    public:
        void handle() override;
    private:
        std::string name() const override;
    public:
        static std::string url();
    };

}
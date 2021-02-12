
#pragma once

#include "User.hpp"
#include "LoginData.hpp"
#include "JsonMapper.hpp"


namespace habit {

    using LoginData = net::LoginData;

    MAKE_CLASS_INFO(LoginData,
                    MAKE_PROPERTY(LoginData, login),
                    MAKE_PROPERTY(LoginData, password)
    );

    MAKE_CLASS_INFO(User,
                    MAKE_PROPERTY(User, email),
                    MAKE_SECURE_PROPERTY(User, password),
                    MAKE_PROPERTY(User, name),
                    MAKE_PROPERTY(User, surname)
    );

    MAKE_MAPPER(mapper,
                InfoOfLoginData,
                InfoOfUser);

    inline constexpr auto json_mapper = mapping::JSONMapper<mapper>();
    using JSON = mapping::JSON;

}


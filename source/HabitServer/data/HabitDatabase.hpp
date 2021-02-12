
#pragma once

#include "Database.hpp"
#include "HabitMapping.hpp"

namespace habit {

    inline constexpr auto sqlite_mapper = sql::SQLiteMapper<habit::mapper>();

    using Database = sql::Database<sqlite_mapper>;

    static inline auto& database() {
        static auto instance = Database("test_db1.sql");
        return instance;
    };

};

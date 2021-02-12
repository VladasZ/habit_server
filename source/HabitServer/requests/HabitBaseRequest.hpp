//
// Created by Vladas Zakrevskis on 05/02/2020.
//

#pragma once

#include "BaseRequest.hpp"
#include "HabitMapping.hpp"

namespace habit {
	template <class T = std::string>
	class HabitBaseRequest : public BaseRequest<T, habit::json_mapper> { };
}

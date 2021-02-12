
#pragma once

#include <Poco/Util/ServerApplication.h>

namespace habit {
	class Server : public Poco::Util::ServerApplication {
	private:
		int main(const std::vector<std::string>&) override;
	};
}

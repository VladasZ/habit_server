
#include <Poco/Net/HTTPServer.h>
#include <Poco/Net/HTTPServerParams.h>

#include "Log.hpp"
#include "HabitServer.hpp"
#include "HabitDatabase.hpp"
#include "ExceptionCatch.hpp"
#include "HabitRequestHandler.hpp"

using namespace std;
using namespace habit;
using namespace Poco::Net;


int Server::main(const vector<string>&) {

    try {
        Log << "Server init";

        database().dump_all<json_mapper>();

        HTTPServer server(new HabitRequestHandler, ServerSocket(80), new HTTPServerParams);
        server.start();
        Log << "Server started";
        waitForTerminationRequest();

        Log << "Shutting down...";
        server.stop();
        return Server::EXIT_OK;
    }
    catch (...) {
        Log << cu::what();
        return Server::EXIT_IOERR;
    }

}

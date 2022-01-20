#pragma once

#include <filesystem>

#include "problem.h"

struct Compiler {
    std::string command;
    std::string flags;
};

const Compiler GXX {
    .command = "g++",
    .flags = "-D_JUDGE_ -DNDEBUG -O2"
};

const Compiler P1XX {
    .command = "g++",
    .flags = "-D_JUDGE_ -DNDEBUG -O2 -Wall -Wextra -Werror -Wno-sign-compare -Wshadow"
};

bool compile_problem(const Problem& p, const std::filesystem::path& templates_dir);
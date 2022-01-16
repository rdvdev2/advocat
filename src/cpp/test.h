#pragma once

#include <filesystem>
#include <vector>

#include "problem.h"

struct Test {
    std::filesystem::path inputs;
    std::filesystem::path outputs;
    std::filesystem::path tmpfile;
};

struct Testsuit {
    std::string name;
    std::vector<Test> tests;
};

void find_tests(const std::filesystem::path& folder, const Problem& p, Testsuit& testsuit);

int run_testsuit(const Problem& p, const Testsuit& testsuit);
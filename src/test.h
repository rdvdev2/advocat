#pragma once

#include <filesystem>
#include <vector>

#include "problem.h"

struct Test {
    std::filesystem::path inputs;
    std::filesystem::path outputs;
    std::filesystem::path tmpfile;
};

typedef std::vector<Test> Testsuit;

void find_tests(const std::filesystem::path& folder, Testsuit& tests);

int run_testsuit(const std::string& suitname, const Testsuit& tests, const Problem& p);
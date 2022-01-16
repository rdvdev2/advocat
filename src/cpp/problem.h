#pragma once

#include <string>
#include <filesystem>
#include <vector>

struct Problem {
    std::string id;
    std::filesystem::path source;
    std::filesystem::path output;

    bool is_private;
    bool has_main;

    std::string zip_url;
    std::string main_cc_url;

    std::filesystem::path advocat_dir;
};

void generate_problem(Problem& p, const std::filesystem::path& folder);

std::string verify_problem(const Problem& p);

void gather_problem_info(Problem& p);
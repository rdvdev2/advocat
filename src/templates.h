#pragma once

#include <filesystem>

#include "problem.h"

void apply_template(const std::string& template_name, const std::filesystem::path& templates_dir, const std::filesystem::path& output, const Problem& p);
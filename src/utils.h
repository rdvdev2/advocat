#pragma once

#include <fstream>
#include <filesystem>

void read_file(const std::filesystem::path& file, std::string& contents);

void show_error(const std::string& description);
void show_warning(const std::string& description);

void show_progress(const std::string& name, char result);
void show_result(const std::string& name, char veredict);

void show_details(const std::string& title, const std::string& details);
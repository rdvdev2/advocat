#pragma once

#include <fstream>
#include <filesystem>

enum struct TaskType { Fetch, Test };
enum struct TaskStatus { Done, Pass, SkipGood, SkipBad, Fail, InProgress };

void read_file(const std::filesystem::path& file, std::string& contents);

void show_error(const std::string& description);
void show_warning(const std::string& description);

void show_task_status(std::string name, TaskType type, TaskStatus status);

void show_details(const std::string& title, const std::string& details);
#pragma once

#include <fstream>
#include <filesystem>

enum struct MessageType { Debug = 3, Info = 2, Warning = 1, Error = 0 };

enum struct TaskType { Fetch, Test };
enum struct TaskStatus { Done, Pass, SkipGood, SkipBad, Fail, InProgress };

void read_file(const std::filesystem::path& file, std::string& contents);

int run_system_command(const std::string& command);

void print_message(MessageType type, const std::string& message);
#define DEBUG(msg) print_message(MessageType::Debug, msg)
#define INFO(msg) print_message(MessageType::Info, msg)
#define WARN(msg) print_message(MessageType::Warning, msg)
#define ERROR(msg) print_message(MessageType::Error, msg)

void show_task_status(std::string name, TaskType type, TaskStatus status);

void show_details(const std::string& title, const std::string& details);

#ifdef ADVOCAT_DEBUG
    const MessageType LOG_LEVEL = MessageType::Debug;
#else
    const MessageType LOG_LEVEL = MessageType::Info;
#endif
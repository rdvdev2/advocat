
#include <filesystem>

#include "download.h"
#include "utils.h"

using namespace std;

const string WGET_TEMPLATE = "wget -nv -nc -O ";
const string UNZIP_TEMPLATE_0 = "unzip -joq ";
const string UNZIP_TEMPLATE_1 = " \"**/sample*\" -d ";
const string SILENT_TEMPLATE = " > /dev/null 2> /dev/null";

const string DOWNLOAD_ZIP_TEXT = "Downloading tests";
const string DOWNLOAD_MAIN_CC_TEXT = "Downloading main.cc";
const string EXTRACT_TESTS_TEXT = "Extracting tests";

bool download_file(const string& url, const filesystem::path path) {
    DEBUG("Downloading " + url + " to " + path.string());
    string command = WGET_TEMPLATE + path.string() + " " + url + SILENT_TEMPLATE;

    return run_system_command(command) == 0 and filesystem::exists(path);
}

bool unzip_file(const filesystem::path zip_path, const filesystem::path output_path) {
    DEBUG("Extracting " + zip_path.string() + " to " + output_path.string());
    string command = UNZIP_TEMPLATE_0 + zip_path.string() + UNZIP_TEMPLATE_1 + output_path.string() + SILENT_TEMPLATE;

    return run_system_command(command) == 0 and filesystem::exists(output_path);
}

bool download_zip(const Problem& p) {
    show_task_status(DOWNLOAD_ZIP_TEXT, TaskType::Fetch, TaskStatus::InProgress);

    filesystem::path path = p.advocat_dir / "problem.zip";

    if (filesystem::exists(path)) {
        show_task_status(DOWNLOAD_ZIP_TEXT, TaskType::Fetch, TaskStatus::SkipGood);
        return true;
    }

    if (p.is_private) {
        show_task_status(DOWNLOAD_ZIP_TEXT, TaskType::Fetch, TaskStatus::SkipBad);
        return false;
    }

    bool success = download_file(p.zip_url, path);
    show_task_status(DOWNLOAD_ZIP_TEXT, TaskType::Fetch,success ? TaskStatus::Done : TaskStatus::Fail);
    return success;
}

bool download_main_cc(const Problem& p) {
    show_task_status(DOWNLOAD_MAIN_CC_TEXT, TaskType::Fetch, TaskStatus::InProgress);

    if (p.has_main) {
        show_task_status(DOWNLOAD_MAIN_CC_TEXT, TaskType::Fetch, TaskStatus::SkipGood);
        return true;
    }

    filesystem::path path = p.advocat_dir / "main.cc";

    if (filesystem::exists(path)) {
        show_task_status(DOWNLOAD_MAIN_CC_TEXT, TaskType::Fetch, TaskStatus::SkipGood);
        return true;
    }

    if (p.is_private) {
        show_task_status(DOWNLOAD_MAIN_CC_TEXT, TaskType::Fetch, TaskStatus::SkipBad);
        return false;
    }

    bool success = download_file(p.main_cc_url, path);
    show_task_status(DOWNLOAD_MAIN_CC_TEXT, TaskType::Fetch, success ? TaskStatus::Done : TaskStatus::Fail);
    return success;
}

bool extract_tests(const Problem& p) {
    show_task_status(EXTRACT_TESTS_TEXT, TaskType::Fetch, TaskStatus::InProgress);

    filesystem::path zip_path = p.advocat_dir / "problem.zip";
    filesystem::path tests_path = p.advocat_dir / "tests";

    if (filesystem::exists(tests_path)) {
        show_task_status(EXTRACT_TESTS_TEXT, TaskType::Fetch, TaskStatus::SkipGood);
        return true;
    }

    if (not filesystem::exists(zip_path)) {
        show_task_status(EXTRACT_TESTS_TEXT, TaskType::Fetch, TaskStatus::SkipBad);
        return false;
    }

    bool success = unzip_file(zip_path, tests_path);
    show_task_status(EXTRACT_TESTS_TEXT, TaskType::Fetch, success ? TaskStatus::Done : TaskStatus::Fail);
    return success;
}
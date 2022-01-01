
#include <filesystem>

#include "download.h"
#include "utils.h"

using namespace std;

const string WGET_TEMPLATE = "wget -nv -nc -O ";
const string UNZIP_TEMPLATE_0 = "unzip -joq ";
const string UNZIP_TEMPLATE_1 = " \"**/sample*\" -d ";
const string SILENT_TEMPLATE = " > /dev/null 2> /dev/null";

const string DOWNLOAD_ZIP_TEXT = "Downloading tests...";
const string DOWNLOAD_MAIN_CC_TEXT = "Downloading main.cc...";
const string EXTRACT_TESTS_TEXT = "Extracting tests...";

bool download_file(const string& url, const filesystem::path path) {
    string command = WGET_TEMPLATE + path.string() + " " + url + SILENT_TEMPLATE;
    system(command.c_str());
    return filesystem::exists(path);
}

bool unzip_file(const filesystem::path zip_path, const filesystem::path output_path) {
    string command = UNZIP_TEMPLATE_0 + zip_path.string() + UNZIP_TEMPLATE_1 + output_path.string() + SILENT_TEMPLATE;
    system(command.c_str());
    return filesystem::exists(output_path);
}

bool download_zip(const Problem& p) {
    show_progress(DOWNLOAD_ZIP_TEXT, 'I');

    filesystem::path path = p.advocat_dir / "problem.zip";

    if (filesystem::exists(path)) {
        show_progress(DOWNLOAD_ZIP_TEXT, 'S');
        return true;
    }

    if (p.is_private) {
        show_progress(DOWNLOAD_ZIP_TEXT, 'S');
        return false;
    }

    bool success = download_file(p.zip_url, path);
    show_progress(DOWNLOAD_ZIP_TEXT, success ? 'D' : 'F');
    return success;
}

bool download_main_cc(const Problem& p) {
    show_progress(DOWNLOAD_MAIN_CC_TEXT, 'I');

    if (p.has_main) {
        show_progress(DOWNLOAD_MAIN_CC_TEXT, 'S');
        return true;
    }

    filesystem::path path = p.advocat_dir / "main.cc";

    if (filesystem::exists(path)) {
        show_progress(DOWNLOAD_MAIN_CC_TEXT, 'S');
        return true;
    }

    if (p.is_private) {
        show_progress(DOWNLOAD_MAIN_CC_TEXT, 'S');
        return false;
    }

    bool success = download_file(p.main_cc_url, path);
    show_progress(DOWNLOAD_MAIN_CC_TEXT, success ? 'D' : 'F');
    return success;
}

bool extract_tests(const Problem& p) {
    show_progress(EXTRACT_TESTS_TEXT, 'I');

    filesystem::path zip_path = p.advocat_dir / "problem.zip";
    filesystem::path tests_path = p.advocat_dir / "tests";

    if (filesystem::exists(tests_path)) {
        show_progress(EXTRACT_TESTS_TEXT, 'S');
        return true;
    }

    if (not filesystem::exists(zip_path)) {
        show_progress(EXTRACT_TESTS_TEXT, 'S');
        return false;
    }

    bool success = unzip_file(zip_path, tests_path);
    show_progress(EXTRACT_TESTS_TEXT, success ? 'D' : 'F');
    return success;
}
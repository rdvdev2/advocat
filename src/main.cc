
#include <iostream>
#include <filesystem>
#include <unistd.h>

#include "colors.h"
#include "problem.h"
#include "compiler.h"
#include "download.h"
#include "utils.h"
#include "test.h"

#ifndef APP_VERSION
    #define APP_VERSION "UNKNOWN"
#endif

using namespace std;

const int BUFFSIZE = 1024;

int main() {
    INFO("Advocat v" + string(APP_VERSION) + " by Roger Díaz Viñolas (rdvdev2@gmail.com)");
    DEBUG("Debug mode ON: To supress verbose output remove the --debug flag");

    DEBUG("Searching the binary directory...");
    char buf[BUFFSIZE];
    auto len = readlink("/proc/self/exe", buf, BUFFSIZE);
    if (len != -1) buf[len] = '\0';
    else {
        ERROR("Can't find the templates");
        return 1;
    }
    filesystem::path binary_dir = filesystem::path(buf).parent_path();
    DEBUG("Found the binary on " + binary_dir.string());

    Problem p;
    filesystem::path cwd = filesystem::current_path();
    generate_problem(p, cwd);

    string error = verify_problem(p);
    if (not error.empty()) {
        ERROR(error);
        return 1;
    }

    gather_problem_info(p);
    if (not filesystem::exists(p.advocat_dir)) {
        DEBUG("Creating the problem directory: " + p.advocat_dir.string());
        filesystem::create_directories(p.advocat_dir);
    }

    if (p.is_private) {
        cout << endl;
        WARN("This problem isn't public! No tests or main() will be downloaded!");
        cout << endl;
    }

    bool zip = download_zip(p);
    bool main_cc = download_main_cc(p);
    bool tests = extract_tests(p);

    if (not zip and p.is_private) {
        cerr << endl;
        WARN("Unable to retrieve tests!");
        cerr << "You can manually download the problem zip from [" << p.zip_url << "] and save it as [" << p.advocat_dir.string() << "/problem.zip]." << endl;
    }

    if (not main_cc) {
        cerr << endl;
        ERROR("Unable to retrive the main.cc file!");
        cerr << "You can manually download the main.cc file from [" << p.main_cc_url << "] and save it as [" << p.advocat_dir.string() << "/main.cc]." << endl;
    }

    if (not tests and not p.is_private) {
        cerr << endl;
        WARN("Unable to unzip tests!");
    }

    DEBUG("Searching for tests...");
    Testsuit public_testsuit, user_testsuit;
    public_testsuit.name = "public";
    user_testsuit.name = "user";

    if (tests) find_tests(p.advocat_dir / "tests", p, public_testsuit);
    find_tests(cwd, p, user_testsuit);
    DEBUG("Test search finished");

    int test_count = public_testsuit.tests.size() + user_testsuit.tests.size();
    if (test_count == 0) {
        cerr << endl;
        WARN("No tests were found!");
    }
    
    cout << endl;
    INFO("Compiling and running tests...");

    bool compiles = compile_problem(p, binary_dir);
    int pass_count = run_testsuit(p, public_testsuit);
    pass_count += run_testsuit(p, user_testsuit);

    cout << endl;
    if (not compiles) {
        cout << RED << "Your code doesn't compile!";
    } else if (public_testsuit.tests.empty() and user_testsuit.tests.empty()) {
        cout << ORANGE << "Your code compiles but you should test it before submitting. Try to add some tests to the folder.";
    } else if (pass_count != test_count) {
        cout << RED << "DON'T submit your code to jutge.org!";
    } else {
        cout << GREEN << "You're ready to submit your code to jutge.org!";
    }
    cout << " (" << pass_count << " out of " << test_count << " tests passed)" << NO_COLOR << endl;

    DEBUG("Clean end! Hooray!");
}
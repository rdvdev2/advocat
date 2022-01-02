
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
    cout << "Advocat v" << APP_VERSION << " by Roger Díaz Viñolas (rdvdev2@gmail.com)" << endl;

    char buf[BUFFSIZE];
    auto len = readlink("/proc/self/exe", buf, BUFFSIZE);
    if (len != -1) buf[len] = '\0';
    else {
        show_error("Can't find the templates");
        return 1;
    }
    filesystem::path binary_dir = filesystem::path(buf).parent_path();

    Problem p;
    filesystem::path cwd = filesystem::current_path();
    generate_problem(p, cwd);

    string error = verify_problem(p);
    if (not error.empty()) {
        show_error(error);
        return 1;
    }

    gather_problem_info(p);
    if (not filesystem::exists(p.advocat_dir)) filesystem::create_directories(p.advocat_dir);

    if (p.is_private) {
        cout << endl;
        show_warning("This problem isn't public! No tests or main() will be downloaded!");
        cout << endl;
    }

    bool zip = download_zip(p);
    bool main_cc = download_main_cc(p);
    bool tests = extract_tests(p);

    if (not zip and p.is_private) {
        cerr << endl;
        show_warning("Unable to retrieve tests!");
        cerr << "You can manually download the problem zip from [" << p.zip_url << "] and save it as [" << p.advocat_dir.string() << "/problem.zip]." << endl;
    }

    if (not main_cc) {
        cerr << endl;
        show_error("Unable to retrive the main.cc file!");
        cerr << "You can manually download the main.cc file from [" << p.main_cc_url << "] and save it as [" << p.advocat_dir.string() << "/main.cc]." << endl;
    }

    if (not tests and not p.is_private) {
        cerr << endl;
        show_warning("Unable to unzip tests!");
    }

    Testsuit problem_tests, user_tests;
    find_tests(p.advocat_dir / "tests", problem_tests);
    find_tests(cwd, user_tests);

    int test_count = problem_tests.size() + user_tests.size();
    if (test_count == 0) {
        cerr << endl;
        show_warning("No tests were found!");
    }
    
    cout << endl << "Compiling and running tests..." << endl;

    bool compiles = compile_problem(p, binary_dir);
    int pass_count = run_testsuit("PUBLIC TEST", problem_tests, p);
    pass_count += run_testsuit("USER TEST", user_tests, p);

    cout << endl;
    if (not compiles) {
        cout << RED << "Your code doesn't compile!";
    } else if (problem_tests.empty() and user_tests.empty()) {
        cout << ORANGE << "Your code compiles but you should test it before submitting. Try to add some tests to the folder.";
    } else if (pass_count != test_count) {
        cout << RED << "DON'T submit your code to jutge.org!";
    } else {
        cout << GREEN << "You're ready to submit your code to jutge.org!";
    }
    cout << " (" << pass_count << " out of " << test_count << " tests passed)" << NO_COLOR << endl;
}
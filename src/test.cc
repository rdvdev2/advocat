
#include <algorithm>

#include "utils.h"
#include "test.h"

using namespace std;

int compare_tests(const Test& a, const Test& b) {
    return a.inputs.stem().string() < b.inputs.stem().string();
}

void find_tests(const filesystem::path& folder, const Problem& p, Testsuit& testsuit) {
    DEBUG("Searching for tests in " + folder.string());

    for (auto& file: filesystem::recursive_directory_iterator(folder)) {
        if (file.path().extension() == ".inp") {
            Test t {
                .inputs = file.path(),
                .outputs = file.path(),
                .tmpfile = filesystem::temp_directory_path() / "advocat" / p.id / testsuit.name / file.path().filename()
            };
            t.outputs.replace_extension(".cor");
            t.tmpfile.replace_extension(".out");
            
            if (filesystem::exists(t.outputs)) {
                testsuit.tests.push_back(t);
                DEBUG("Test found! Details:");
                DEBUG("-> inputs: " + t.inputs.string());
                DEBUG("-> outputs: " + t.outputs.string());
                DEBUG("-> tmpfile: " + t.tmpfile.string());
            }
        }
    }

    DEBUG("Sorting tests...");
    sort(testsuit.tests.begin(), testsuit.tests.end(), compare_tests);
    DEBUG("Tests sorted");
}

int run_testsuit(const Problem& p, const Testsuit& testsuit) {
    int pass_count = 0;
    int test_count = testsuit.tests.size();

    for (int i = 0; i < test_count; ++i) {
        string testname = testsuit.name + " test " + to_string(i+1);
        show_task_status(testname, TaskType::Test, TaskStatus::InProgress);

        if (not filesystem::exists(p.output)) {
            show_task_status(testname, TaskType::Test, TaskStatus::SkipBad);
            continue;
        }

        if (filesystem::exists(testsuit.tests[i].tmpfile)) {
            DEBUG("Removing previous output: " + testsuit.tests[i].tmpfile.string());
            filesystem::remove(testsuit.tests[i].tmpfile);
        }

        if (not filesystem::exists(testsuit.tests[i].tmpfile.parent_path())) {
            DEBUG("Creating folder for test output: " + testsuit.tests[i].tmpfile.parent_path().string());
            filesystem::create_directories(testsuit.tests[i].tmpfile.parent_path());
        }
        
        DEBUG("Running test...");
        string command = p.output.string() + " < " + testsuit.tests[i].inputs.string() + " > " + testsuit.tests[i].tmpfile.string();
        run_system_command(command);

        filesystem::path diff = testsuit.tests[i].tmpfile;
        diff.replace_extension("diff");

        DEBUG("Verifying output...");
        command = "diff -y " + testsuit.tests[i].outputs.string() + " " + testsuit.tests[i].tmpfile.string() + " > " + diff.string();
        int diff_ret = run_system_command(command);

        
        if (diff_ret == 0) {
            show_task_status(testname, TaskType::Test, TaskStatus::Pass);
            ++pass_count;
        } else {
            show_task_status(testname, TaskType::Test, TaskStatus::Fail);

            DEBUG("Getting output diff...");

            string diff_contents;
            read_file(diff, diff_contents);

            show_details("Expected output vs your output", diff_contents);
        }
    }

    return pass_count;
}
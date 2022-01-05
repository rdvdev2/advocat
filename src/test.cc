
#include <algorithm>

#include "utils.h"
#include "test.h"

using namespace std;

int compare_tests(const Test& a, const Test& b) {
    return a.inputs.stem().string() < b.inputs.stem().string();
}

void find_tests(const filesystem::path& folder, Testsuit& tests) {
    for (auto& file: filesystem::recursive_directory_iterator(folder)) {
        if (file.path().extension() == ".inp") {
            Test t {
                .inputs = file.path(),
                .outputs = file.path(),
                .tmpfile = filesystem::temp_directory_path() / "sample.out"
            };
            t.outputs.replace_extension(".cor");
            t.tmpfile.replace_extension(".out");
            
            if (filesystem::exists(t.outputs)) tests.push_back(t);
        }
    }

    sort(tests.begin(), tests.end(), compare_tests);
}

int run_testsuit(const string& suitname, const Testsuit& tests, const Problem& p) {
    int pass_count = 0;
    int test_count = tests.size();

    for (int i = 0; i < test_count; ++i) {
        string testname = suitname + " " + to_string(i+1);
        show_task_status(testname, TaskType::Test, TaskStatus::InProgress);

        if (not filesystem::exists(p.output)) {
            show_task_status(testname, TaskType::Test, TaskStatus::SkipBad);
            continue;
        }
        
        string command = p.output.string() + " < " + tests[i].inputs.string() + " > " + tests[i].tmpfile.string();
        system(command.c_str());

        filesystem::path diff = filesystem::temp_directory_path() / "sample.diff";

        command = "diff -q " + tests[i].outputs.string() + " " + tests[i].tmpfile.string() + " > " + diff.string();
        system(command.c_str());

        string diff_contents;
        read_file(diff, diff_contents);
        
        if (diff_contents.empty()) {
            show_task_status(testname, TaskType::Test, TaskStatus::Pass);
            ++pass_count;
        } else {
            show_task_status(testname, TaskType::Test, TaskStatus::Fail);

            command = "diff -y " + tests[i].outputs.string() + " " + tests[i].tmpfile.string() + " > " + diff.string();
            system(command.c_str());
            read_file(diff, diff_contents);

            show_details("Expected output vs your output", diff_contents);
        }
    }

    return pass_count;
}
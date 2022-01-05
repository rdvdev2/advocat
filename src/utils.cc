
#include <iostream>
#include <algorithm>
#include <string>

#include "utils.h"
#include "colors.h"

using namespace std;

void read_file(const filesystem::path& file, string& contents) {
    ifstream input_file(file);
    contents = string(istreambuf_iterator<char>(input_file), istreambuf_iterator<char>());
}

void show_error(const string& description) {
    cerr << RED << "ERROR: " << description << NO_COLOR << endl;
}

void show_warning(const string& description) {
    cerr << ORANGE << "WARNING: " << description << NO_COLOR << endl;
}

void show_task_status(string name, TaskType type, TaskStatus status) {
    switch (type) {
        case TaskType::Fetch: name.append("... "); break;
        case TaskType::Test: transform(name.begin(), name.end(), name.begin(), ::toupper); name.append(": "); break;
    }
    cout << YELLOW << name;

    switch (status) {
        case TaskStatus::Done:       cout << GREEN    << "DONE ✓" << NO_COLOR << endl; break;
        case TaskStatus::Pass:       cout << GREEN    << "PASS ✓" << NO_COLOR << endl; break;
        case TaskStatus::SkipGood:   cout << CYAN     << "SKIP ✓" << NO_COLOR << endl; break;
        case TaskStatus::SkipBad:    cout << CYAN     << "SKIP ✘" << NO_COLOR << endl; break;
        case TaskStatus::Fail:       cout << RED      << "FAIL ✘" << NO_COLOR << endl; break;
        case TaskStatus::InProgress: cout << NO_COLOR << "... \r" << flush;            break;
    }
}

void show_details(const string& title, const string& details) {
    cout  << "==> " << title  << ":" << endl << PURPLE;
    stringstream details_stream(details);
    string line;
    while (getline(details_stream, line)) {
        cout << line << endl;
    }
    cout << NO_COLOR;
}
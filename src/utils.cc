
#include <iostream>
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

// D: Done; S: Skip; F: Fail; I: In progress
void show_progress(const string& name, char result) {
    switch (result) {
        case 'D': cout << YELLOW << name << GREEN << " DONE ✓" << NO_COLOR << endl; break;
        case 'S': cout << YELLOW << name << CYAN  << " SKIP ✓" << NO_COLOR << endl; break;
        case 'F': cout << YELLOW << name << RED   << " FAIL ✘" << NO_COLOR << endl; break;
        case 'I': cout << YELLOW << name << NO_COLOR << "\r" << flush; break;
    }
}

// P: Pass; S: Skip; F: Fail; I: In progress
void show_result(const string& name, char veredict) {
    switch (veredict) {
        case 'P': cout << YELLOW << name << ": " << GREEN << "PASS ✓" << NO_COLOR << endl; break;
        case 'S': cout << YELLOW << name << ": " << CYAN  << "SKIP ✘" << NO_COLOR << endl; break;
        case 'F': cout << YELLOW << name << ": " << RED   << "FAIL ✘" << NO_COLOR << endl; break;
        case 'I': cout << YELLOW << name << ": " << NO_COLOR << "..." << "\r" << flush; break;
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
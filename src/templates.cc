
#include <fstream>

#include "templates.h"

using namespace std;

void copy_file_contents(ofstream& output, ifstream& input) {
    if (input.is_open()) {
        string line;
        while (getline(input, line)) {
            output << line << "\n";
        }
    }
}

void apply_template(const string& template_name, const filesystem::path& templates_dir, const filesystem::path& output, const Problem& p) {
    filesystem::path template_path = templates_dir / template_name;
    filesystem::path stub_path = templates_dir / "stub.cc.in";
    filesystem::path main_path = p.advocat_dir / "main.cc";

    if (filesystem::exists(output)) filesystem::remove(output);

    ifstream template_file(template_path);
    ifstream stub_file(stub_path);
    ifstream original_file(p.source);
    ifstream main_file(main_path);
    ofstream output_file(output);

    if (template_file.is_open() and output_file.is_open()) {
        string template_line;
        while (getline(template_file, template_line)) {
            if (template_line.find("{original}") != string::npos) {
                copy_file_contents(output_file, original_file);
            } else if (template_line.find("{main}") != string::npos) {
                copy_file_contents(output_file, main_file);
            } else if (template_line.find("{stub}") != string::npos) {
                copy_file_contents(output_file, stub_file);
            } else {
                output_file << template_line << "\n";
            }
        }
    }

    template_file.close();
    stub_file.close();
    original_file.close();
    main_file.close();

    output_file.flush();
    output_file.close();
}
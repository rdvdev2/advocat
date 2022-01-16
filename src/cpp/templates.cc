
#include <fstream>

#include "templates.h"
#include "utils.h"

using namespace std;

void copy_file_contents(ofstream& output, ifstream& input) {
    if (input.is_open()) {
        string line;
        while (getline(input, line)) {
            output << line << "\n";
        }
        DEBUG("File included successfully");
    } else {
        DEBUG("Couldn't open file for inclusion, ignoring it");
    }
}

void apply_template(const string& template_name, const filesystem::path& templates_dir, const filesystem::path& output, const Problem& p) {
    filesystem::path template_path = templates_dir / template_name;
    filesystem::path stub_path = templates_dir / "stub.cc.in";
    filesystem::path main_path = p.advocat_dir / "main.cc";

    DEBUG("Generating source code from template " + template_path.string() + " and saving it to " + output.string());

    if (filesystem::exists(output)) {
        DEBUG("Removing previously generated source code: " + output.string());
        filesystem::remove(output);
    }

    DEBUG("Opening needed files...");
    ifstream template_file(template_path);
    ifstream stub_file(stub_path);
    ifstream original_file(p.source);
    ifstream main_file(main_path);
    ofstream output_file(output);

    DEBUG("Parsing templates and writing...");
    if (template_file.is_open() and output_file.is_open()) {
        string template_line;
        while (getline(template_file, template_line)) {
            if (template_line.find("{original}") != string::npos) {
                DEBUG("Including file " + p.source.string());
                copy_file_contents(output_file, original_file);
            } else if (template_line.find("{main}") != string::npos) {
                DEBUG("Including file " + main_path.string());
                copy_file_contents(output_file, main_file);
            } else if (template_line.find("{stub}") != string::npos) {
                DEBUG("Including template " + stub_path.string());
                copy_file_contents(output_file, stub_file);
            } else {
                output_file << template_line << "\n";
            }
        }
    }

    DEBUG("Closing used files...");
    template_file.close();
    stub_file.close();
    original_file.close();
    main_file.close();

    output_file.flush();
    output_file.close();

    DEBUG("Done writing source code to " + output.string());
}
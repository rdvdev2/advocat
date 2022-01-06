
#include "compiler.h"
#include "utils.h"
#include "templates.h"

using namespace std;

const string COMPILATION_TEXT = "Compilation";

bool compile_file(const Compiler& compiler, const filesystem::path& source, const filesystem::path& output) {
    DEBUG("Compiling " + source.string() + " to " + output.string());
    if (not filesystem::exists(source)) {
        DEBUG(source.string() + " doesn't exist!");
        return false;
    }
    filesystem::path errors = output.parent_path() / "compilation.err";
    
    if (filesystem::exists(output)) {
        DEBUG("Removing previous compilation output: " + output.string());
        filesystem::remove(output);
    }
    string command = compiler.command + " " + compiler.flags + " -c " + source.string() + " -o " + output.string() + " 2> " + errors.string();
    run_system_command(command);

    return filesystem::exists(output);
}

bool link_file(const Compiler& compiler, const filesystem::path& source, const filesystem::path& output) {
    DEBUG("Linking " + source.string() + " to " + output.string());
    if (not filesystem::exists(source)) {
        DEBUG(source.string() + " doesn't exist!");
        return false;
    }
    filesystem::path errors = output.parent_path() / "compilation.err";

    if (filesystem::exists(output)) {
        DEBUG("Removing previous linking output: " + output.string());
        filesystem::remove(output);
    }
    string command = compiler.command + " " + compiler.flags + " " + source.string() + " -o " + output.string() + " 2> " + errors.string();;
    run_system_command(command);

    return filesystem::exists(output);
}

bool check_p1xx_compiles(const Problem& p) {
    DEBUG("Checking if P1++ compiles the user's main.cc...");
    filesystem::path object_file = p.advocat_dir / "main.o";

    return compile_file(P1XX, p.source, object_file);
}

bool compile_binary(const Problem& p, const filesystem::path& templates_dir) {
    DEBUG("Compiling the binary for testing with G++...");
    filesystem::path joined_source = p.advocat_dir / "joined.cc";

    string template_name;
    if (p.has_main) template_name = "normal.cc.in";
    else template_name = "nomain.cc.in";
    apply_template(template_name, templates_dir, joined_source, p);

    filesystem::path object_file = p.advocat_dir / "joined.o";

    return compile_file(GXX, joined_source, object_file) and link_file(GXX, object_file, p.output);
}

bool compile_problem(const Problem& p, const filesystem::path& templates_dir) {
    show_task_status(COMPILATION_TEXT, TaskType::Test, TaskStatus::InProgress);

    bool compiles = check_p1xx_compiles(p) and compile_binary(p, templates_dir);

    show_task_status(COMPILATION_TEXT, TaskType::Test, compiles ? TaskStatus::Pass : TaskStatus::Fail);

    if (not compiles) {
        string details;
        read_file(p.advocat_dir / "compilation.err", details);
        show_details("Compilation output", details);
    }

    return compiles;
}
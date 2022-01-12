
#include "compiler.h"
#include "utils.h"
#include "templates.h"

using namespace std;

const string COMPILATION_TEXT = "Compilation";

bool compile_file(const Compiler& compiler, const filesystem::path& source, const filesystem::path& output, const filesystem::path& errors) {
    DEBUG("Compiling " + source.string() + " to " + output.string());
    if (not filesystem::exists(source)) {
        DEBUG(source.string() + " doesn't exist!");
        return false;
    }
    
    if (filesystem::exists(output)) {
        DEBUG("Removing previous compilation output: " + output.string());
        filesystem::remove(output);
    }

    if (filesystem::exists(errors)) {
        DEBUG("Removing previous compilation error output: " + errors.string());
        filesystem::remove(errors);
    }

    string command = compiler.command + " " + compiler.flags + " -c " + source.string() + " -o " + output.string() + " 2> " + errors.string();
    
    return run_system_command(command) == 0 and filesystem::exists(output);
}

bool link_file(const Compiler& compiler, const filesystem::path& source, const filesystem::path& output, const filesystem::path& errors) {
    DEBUG("Linking " + source.string() + " to " + output.string());
    if (not filesystem::exists(source)) {
        DEBUG(source.string() + " doesn't exist!");
        return false;
    }

    if (filesystem::exists(output)) {
        DEBUG("Removing previous linking output: " + output.string());
        filesystem::remove(output);
    }

    if (filesystem::exists(errors)) {
        DEBUG("Removing previous linking error output: " + errors.string());
        filesystem::remove(errors);
    }

    string command = compiler.command + " " + compiler.flags + " " + source.string() + " -o " + output.string() + " 2> " + errors.string();;
    
    return run_system_command(command) == 0 and filesystem::exists(output);
}

bool check_p1xx_compiles(const Problem& p, filesystem::path& errors) {
    DEBUG("Checking if P1++ compiles the user's main.cc...");
    filesystem::path object_file = p.advocat_dir / "main.o";
    errors = p.advocat_dir / "p1xx-compilation.err";

    return compile_file(P1XX, p.source, object_file, errors);
}

bool compile_binary(const Problem& p, const filesystem::path& templates_dir, filesystem::path& errors) {
    DEBUG("Compiling the binary for testing with G++...");
    filesystem::path joined_source = p.advocat_dir / "joined.cc";

    string template_name;
    if (p.has_main) template_name = "normal.cc.in";
    else template_name = "nomain.cc.in";
    apply_template(template_name, templates_dir, joined_source, p);

    filesystem::path object_file = p.advocat_dir / "joined.o";

    errors = p.advocat_dir / "c1xx-compilation.err";
    bool compiles = compile_file(GXX, joined_source, object_file, errors);
    if (not compiles) return false;

    errors = p.advocat_dir / "c1xx-linking.err";
    return link_file(GXX, object_file, p.output, errors);
}

bool compile_problem(const Problem& p, const filesystem::path& templates_dir) {
    show_task_status(COMPILATION_TEXT, TaskType::Test, TaskStatus::InProgress);

    filesystem::path errors;
    bool compiles = check_p1xx_compiles(p, errors) and compile_binary(p, templates_dir, errors);

    show_task_status(COMPILATION_TEXT, TaskType::Test, compiles ? TaskStatus::Pass : TaskStatus::Fail);

    if (not compiles) {
        string details;
        read_file(errors, details);
        show_details("Compilation output", details);
    }

    return compiles;
}
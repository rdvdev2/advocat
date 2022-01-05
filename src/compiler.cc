
#include "compiler.h"
#include "utils.h"
#include "templates.h"

using namespace std;

const string COMPILATION_TEXT = "Compilation";

bool compile_file(const Compiler& compiler, const filesystem::path& source, const filesystem::path& output) {
    if (not filesystem::exists(source)) return false;
    filesystem::path errors = output.parent_path() / "compilation.err";
    
    if (filesystem::exists(output)) filesystem::remove(output);
    string command = compiler.command + " " + compiler.flags + " -c " + source.string() + " -o " + output.string() + " 2> " + errors.string();
    system(command.c_str());

    return filesystem::exists(output);
}

bool link_file(const Compiler& compiler, const filesystem::path& source, const filesystem::path& output) {
    if (not filesystem::exists(source)) return false;
    filesystem::path errors = output.parent_path() / "compilation.err";

    if (filesystem::exists(output)) filesystem::remove(output);
    string command = compiler.command + " " + compiler.flags + " " + source.string() + " -o " + output.string() + " 2> " + errors.string();;
    system(command.c_str());

    return filesystem::exists(output);
}

bool check_p1xx_compiles(const Problem& p) {
    filesystem::path object_file = p.advocat_dir / "main.o";

    return compile_file(P1XX, p.source, object_file);
}

bool compile_binary(const Problem& p, const filesystem::path& templates_dir) {
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
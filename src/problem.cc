
#include <regex>
#include <fstream>
#include <stdexcept>

#include "problem.h"
#include "utils.h"

using namespace std;

const regex PROBLEM_ID_REGEX("(G|P|X)\\d{5}_(?:ca|en|es)");
const regex MAIN_REGEX("int\\s+main\\s*(\\s*)");

const string BASE_PROBLEM_URL = "https://jutge.org/problems/";

void generate_problem(Problem& p, const filesystem::path& folder) {
    p.id = folder.filename().string();

    p.source = folder/ "main.cc";
    p.output = folder / "main.x";

    p.advocat_dir = filesystem::path(getenv("HOME")) / ".advocat" / "problems"/ p.id;

    string problem_url = BASE_PROBLEM_URL + p.id;
    p.zip_url = problem_url + "/zip";
    p.main_cc_url = problem_url + "/main/cc";
}

string verify_problem(const Problem& p) {
    if (not regex_match(p.id, PROBLEM_ID_REGEX)) {
        return "This folder doesn't have the name of a problem id!";
    }
    if (p.id[0] == 'G') {
        return "Game problems aren't supported!";
    }
    if (not filesystem::exists(p.source)) {
        return "This folder doesn't contain a main.cc file!";
    }
    return "";
}

void gather_problem_info(Problem& p) {
    p.is_private = p.id[0] == 'X';

    string main;
    read_file(p.source, main);
    p.has_main = regex_search(main, MAIN_REGEX);
}
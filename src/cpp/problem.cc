
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

    DEBUG("Problem details:");
    DEBUG("-> id: " + p.id);
    DEBUG("-> source: " + p.source.string());
    DEBUG("-> output: " + p.output.string());
    DEBUG("-> advocat_dir: " + p.advocat_dir.string());
    DEBUG("-> zip_url: " + p.zip_url);
    DEBUG("-> main_cc_url: " + p.main_cc_url);
}

string verify_problem(const Problem& p) {
    DEBUG("Verifying the problem...");
    if (not regex_match(p.id, PROBLEM_ID_REGEX)) {
        return "This folder doesn't have the name of a problem id!";
    }
    if (p.id[0] == 'G') {
        return "Game problems aren't supported!";
    }
    if (not filesystem::exists(p.source)) {
        return "This folder doesn't contain a main.cc file!";
    }
    DEBUG("Problem verifyed!");
    return "";
}

void gather_problem_info(Problem& p) {
    p.is_private = p.id[0] == 'X';

    string main;
    read_file(p.source, main);
    p.has_main = regex_search(main, MAIN_REGEX);

    DEBUG("More problem details: ");
    DEBUG("-> is_private: " + string(p.is_private ? "true" : "false"));
    DEBUG("-> has_main: " + string(p.has_main ? "true" : "false"));
}
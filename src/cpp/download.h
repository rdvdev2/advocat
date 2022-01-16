#pragma once

#include "problem.h"

bool download_zip(const Problem& p);
bool download_main_cc(const Problem& p);

bool extract_tests(const Problem& p);
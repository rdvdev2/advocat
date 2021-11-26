#!/bin/bash

AV_VERSION="1.1"
AV_COMPILER="$(which p1++)"
AV_DIR="${HOME}/.advocat"

AV_RED='\033[1;31m'
AV_GREEN='\033[1;32m'
AV_YELLOW="\033[1;33m"
AV_PURPLE="\033[0;35m"
AV_CYAN="\033[1;36m"
AV_ORANGE="\033[0;33m"
AV_NC='\033[0m' # No Color

_advocat_run_test () {
    local INDEX=$1
    local BINARY=$2
    local INPUTS=$3
    local OUTPUTS=$4
    local SKIP=$5

    if [ "${INDEX}" -lt 0 ]; then
        echo -n "${AV_YELLOW}TEST: "
    else
        echo -n "${AV_YELLOW}TEST ${INDEX}: "
    fi

    if [ "$SKIP" = 1 ]; then
        echo "${AV_CYAN}SKIP ✘${AV_NC}"
        return 0
    else
        ${BINARY} < "${INPUTS}" > "${INPUTS%.*}.out"
        if cmp -s "${OUTPUTS}" "${INPUTS%.*}.out"
        then
            echo "${AV_GREEN}PASS ✓${AV_NC}"
            return 1
        else
            echo "${AV_RED}FAIL ✘${AV_NC}"
            echo "${AV_PURPLE}==> Program output:"
            cat "${TEST_INPUT%.*}.out"
            echo "==> Expected output:"
            cat "${TEST_INPUT%.*}.cor"
            echo -n "${AV_NC}"
            return 0
        fi
    fi
}

advocat () {
    echo "Advocat v${AV_VERSION} by Roger Díaz Viñolas (rdvdev2@gmail.com)"

    # Problem specific variables
    local PROBLEM_ID
    PROBLEM_ID=$(basename "$(pwd)")
    local PROBLEM_FOLDER="${AV_DIR}/${PROBLEM_ID}"
    local SOURCE
    SOURCE="$(pwd)/main.cc"
    local BINARY="${PROBLEM_FOLDER}/main.x"
    
    # Compilation variables
    local COMPILE_COMMAND="${AV_COMPILER} -o ${BINARY} ${SOURCE}"
    local COMPILE_OUTPUT_FILE="${PROBLEM_FOLDER}/compilation.out"

    # Sample download variables
    local SAMPLE_DOWNLOAD_FILE="${PROBLEM_FOLDER}/problem.zip"
    local SAMPLE_DOWNLOAD_URL="https://jutge.org/problems/${PROBLEM_ID}/zip"
    local SAMPLES_FOLDER="${PROBLEM_FOLDER}/samples"

    # main() download variables
    local MAIN_DOWNLOAD_FILE="${PROBLEM_FOLDER}/main.cc"
    local MAIN_DOWNLOAD_URL="https://jutge.org/problems/${PROBLEM_ID}/main/cc"
    local MAIN_GREP_PATTERN="int main()"

    # Check if problem is public
    local PRIVATE_PROBLEM=0
    if ! [ "${PROBLEM_ID:0:1}" = "P" ]; then
        echo
        echo "${AV_ORANGE}WARNING: This problem isn't public! No tests or main() will be downloaded!${AV_NC}"
        echo "You can manually download the problem zip from [${SAMPLE_DOWNLOAD_URL}] and save it as [${SAMPLE_DOWNLOAD_FILE}]."
        echo "If needed, download the provided main.cc from [${MAIN_DOWNLOAD_URL}] and save it as [${MAIN_DOWNLOAD_FILE}]."
        echo "The script will use these files if it finds them"
        echo

        PRIVATE_PROBLEM=1
    fi

    # Create advocat dirs if missing
    mkdir -p "${AV_DIR}"
    mkdir -p "${PROBLEM_FOLDER}"

    # Check if an external main is needed
    local EXTERNAL_MAIN=0
    if ! grep -q "$MAIN_GREP_PATTERN" "$SOURCE"; then
        EXTERNAL_MAIN=1
        COMPILE_COMMAND="${COMPILE_COMMAND} ${MAIN_DOWNLOAD_FILE}"
    fi

    # Downloads
    if [ "${PRIVATE_PROBLEM}" = 0 ]; then

        # Download tests from jutge.org
        echo -n "${AV_YELLOW}Downloading tests from jutge.org... "
        if [ -f "${SAMPLE_DOWNLOAD_FILE}" ]; then
            echo "${AV_CYAN}SKIP ✓${AV_NC}"
        else
            wget -nv -nc -O "${SAMPLE_DOWNLOAD_FILE}" "${SAMPLE_DOWNLOAD_URL}" > /dev/null 2> /dev/null
            if [ -f "${SAMPLE_DOWNLOAD_FILE}" ]; then
                echo "${AV_GREEN}DONE ✓${AV_NC}"
            else
                echo "${AV_RED}FAIL ✘${AV_NC}"
                return
            fi
        fi

        # Download main() from jutge.org
        echo -n "${AV_YELLOW}Downloading main() from jutge.org... "
        if [ "${EXTERNAL_MAIN}" = 0 ]; then
            echo "${AV_CYAN}SKIP ✓${AV_NC}"
        else
            if [ -f "${MAIN_DOWNLOAD_FILE}" ]; then
                echo "${AV_CYAN}SKIP ✓${AV_NC}"
            else
                wget -nv -nc -O "${MAIN_DOWNLOAD_FILE}" "${MAIN_DOWNLOAD_URL}" > /dev/null 2> /dev/null
                if [ -f "${MAIN_DOWNLOAD_FILE}" ]; then
                    echo "${AV_GREEN}DONE ✓${AV_NC}"
                else
                    echo "${AV_RED}FAIL ✘${AV_NC}"
                    return
                fi
            fi
        fi
    fi

    # Extract tests zip
    echo -n "${AV_YELLOW}Extracting tests... "
    if [ -d "${SAMPLES_FOLDER}" ]; then
        echo "${AV_CYAN}SKIP ✓${AV_NC}"
    elif ! [ -f "${SAMPLE_DOWNLOAD_FILE}" ]; then
        echo "${AV_CYAN}SKIP ✓ (No tests will run) ${AV_NC}"
    else
        mkdir -p "${SAMPLES_FOLDER}"
        unzip -joq "${SAMPLE_DOWNLOAD_FILE}" "**/sample*" -d "${SAMPLES_FOLDER}" > /dev/null 2> /dev/null
        if [ -d "${SAMPLES_FOLDER}" ]; then
            echo "${AV_GREEN}DONE ✓${AV_NC}"
        else
            echo "${AV_RED}FAIL ✘${AV_NC}"
            return
        fi
    fi

    # Compile and run
    echo
    echo "Compiling and running tests..."

    # Find the test files
    local SAMPLES=""
    if [ -f "${SAMPLES_FOLDER}/sample.inp" ]; then
        SAMPLES="${SAMPLES_FOLDER}/sample.inp"
    elif [ -f "${SAMPLES_FOLDER}/sample-1.inp" ]; then
        SAMPLES=("${SAMPLES_FOLDER}"/sample-*.inp)
    else
        echo "${AV_ORANGE}WARNING: No tests were found!${AV_NC}"
    fi

    # Recompile the binary
    rm -f "$BINARY"
    local SKIP_TESTS=1
    eval "${COMPILE_COMMAND}" 2> "${COMPILE_OUTPUT_FILE}"
    if [ -f "${BINARY}" ]; then
        echo "${AV_YELLOW}COMPILATION: ${AV_GREEN}PASS ✓${AV_NC}"
        chmod +x "${BINARY}"
        SKIP_TESTS=0
    else
        echo "${AV_YELLOW}COMPILATION: ${AV_RED}FAIL ✘${AV_NC}"
        echo "==> Compilation command:"
        echo "${AV_PURPLE}${COMPILE_COMMAND}${AV_NC}"
        echo "==> Compilation output:${AV_PURPLE}"
        cat "${COMPILE_OUTPUT_FILE}"
        echo -n "${AV_NC}"
    fi

    # Run each test
    local PASS_COUNT=0
    local TEST_COUNT=0
    for TEST_INPUT in ${SAMPLES}
    do
        TEST_COUNT=$((TEST_COUNT + 1))
        _advocat_run_test "$TEST_COUNT" "${BINARY}" "${TEST_INPUT}" "${TEST_INPUT%.*}.out" "${SKIP_TESTS}"
        PASS_COUNT=$((PASS_COUNT + $?))
    done
    echo

    # Give a veredict
    if [ "${SKIP_TESTS}" = 1 ]; then
        echo -n "${AV_RED}Your code doesn't compile! "
    elif [ "${TEST_COUNT}" = 0 ]; then
        echo -n "${AV_ORANGE}Your code compiles but you shoul test it before submitting. "
    elif [ "$PASS_COUNT" = "$TEST_COUNT" ]; then
        echo -n "${AV_GREEN}You're ready to submit your code to jutge.org! "
    else
        echo -n "${AV_RED}DON'T submit your code to jutge.org! "
    fi
    echo "(${PASS_COUNT} out of ${TEST_COUNT} tests passed)${AV_NC}"
}
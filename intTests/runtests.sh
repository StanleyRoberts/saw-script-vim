#!/usr/bin/env bash

################################################################
# Setup environment.

trap "finish" SIGINT

finish() {
  echo "tests passed: ${PASSED_TESTS} / ${NUM_TESTS}"
  if [ "${PASSED_TESTS}" == "${NUM_TESTS}" ]; then
    echo "all tests passed"
    exit 0
  else
    if [ "${FAILED_TESTS}" != 0 ]; then
      echo "${FAILED_TEST_DETAILS}"
    fi
    if [ -z "$DONE" ]; then
      echo "interrupted: $i"
    fi
    exit 1
  fi
}

if [ -z "$TESTBASE" ]; then
  TESTBASE=$(pwd)
  export TESTBASE
fi

if [ "${OS}" == "Windows_NT" ]; then
  export CPSEP=";"
  export DIRSEP="\\"
else
  export CPSEP=":"
  export DIRSEP="/"
fi

# Build the class path. On Windows, Java requires Windows-style paths
# here, even in Cygwin.
#
# Locate rt.jar. This is already a Windows path on windows, so no need
# to 'cygpath' it.
JDK=$(support/find-java-rt-jar.sh)
CP="$JDK"
# Add our bundled .jars to the class path.
for i in "$TESTBASE"/jars/*.jar; do
  if [ "$OS" == "Windows_NT" ]; then
    i=$(cygpath -w "$i")
  fi
  CP=$CP$CPSEP$i
done
export CP

# We need the 'eval's here to interpret the single quotes protecting
# the spaces and semi-colons in the Windows class path.
export SAW="eval cabal run ${CABAL_FLAGS} saw -- -j '$CP'"
export JSS="eval cabal run ${CABAL_FLAGS} jss -- -j '$CP' -c ."

# Figure out what tests to run
if [[ -z "$*" ]]; then
  if [ -z "$DISABLED_TESTS" ]; then
    # File listing tests disabled by default.
    DISABLED_TESTS=disabled_tests.txt
  fi
  # Collect tests not listed in the disabled tests.
  TESTS=""
  for t in test*; do
    if ! grep -q "^$t\$" $DISABLED_TESTS; then
      TESTS="$TESTS $t"
    fi
  done
else
  # Default disabled tests are ignored when specific tests are
  # specified on the command line.
  TESTS=$*
fi

if [ -z "${TEST_TIMEOUT}" ]; then
  TEST_TIMEOUT=500
fi

if [ -z "${XML_FILE}" ]; then
  XML_FILE="results.xml"
fi
XML_TEMP="${XML_FILE}.tmp"

################################################################
# Run tests.

mkdir -p logs
rm -f logs/*

NUM_TESTS=$(echo "$TESTS" | wc -w)
PASSED_TESTS=0
FAILED_TESTS=0
FAILED_TEST_DETAILS="failed:"
export TIMEFORMAT="%R"
TOTAL_TIME=0

for i in $TESTS; do
  # Some nasty bash hacking here to catpure the amount of time taken by the test
  # See http://mywiki.wooledge.org/BashFAQ/032
  START_TIME=$SECONDS
  if [ "${OS}" == "Windows_NT" ]; then
    # ulimit is useless on cygwin :-(  Use the 'timeout' utility instead
    ( cd "$i" || exit 1; (/usr/bin/timeout -k 15 ${TEST_TIMEOUT} sh -vx test.sh > "../logs/$i.log" 2>&1) 2>&1 )
    RES=$?
  else
    ( ulimit -t ${TEST_TIMEOUT}; cd "$i" || exit 1; (sh -vx test.sh > "../logs/$i.log" 2>&1) 2>&1 )
    RES=$?
  fi
  END_TIME=$SECONDS
  TEST_TIME=$((END_TIME - START_TIME))

  if [ $((TEST_TIME >= TEST_TIMEOUT)) == 1 ]; then
    TIMED_OUT=" TIMEOUT"
  else
    TIMED_OUT=""
  fi

  TOTAL_TIME=$((TOTAL_TIME + TEST_TIME))

  if [ $RES == 0 ]; then
    PASSED_TESTS=$(( PASSED_TESTS + 1 ))
    echo "$i: OK (${TEST_TIME}s)"
    echo "  <testcase name=\"${i}\" time=\"${TEST_TIME}\" />" >> ${XML_TEMP}
  else
    FAILED_TESTS=$(( FAILED_TESTS + 1 ))
    echo "$i: FAIL (${TEST_TIME}s${TIMED_OUT})"
    if [ -n "$LOUD" ]; then cat "logs/$i.log"; fi
    FAILED_TEST_DETAILS+=" $i"
    {
      echo "  <testcase name=\"${i}\" time=\"${TEST_TIME}\"><failure><![CDATA["
      sed -e 's/]]>/] ]>/' "logs/$i.log"
      echo "]]></failure></testcase>"
    } >> ${XML_TEMP}
  fi
done

DONE=1

echo "<?xml version='1.0'?>" > $XML_FILE
{
  echo "<testsuites errors=\"${FAILED_TESTS}\" tests=\"${NUM_TESTS}\" time=\"${TOTAL_TIME}\">"
  echo " <testsuite name=\"SAWScript Integration Tests\">"
  cat $XML_TEMP
  echo " </testsuite>"
  echo "</testsuites>"
} >> $XML_FILE

rm $XML_TEMP

finish;

#!/opt/homebrew/bin/fish

echo "import $argv[1] -> I $argv[2] C $argv[3]"

set JSON (cat $argv[1] | string trim)
set PARAMS (echo "[$JSON]'")

echo "calling golem instance invoke-and-await import-all"

golem instance invoke-and-await \
    -i $argv[2] \
    -c $argv[3] \
    -j $PARAMS \
    -f mooving:moovables/api/import-all \
    -F json \
    -V 3


#!/bin/bash

## Prerequisites: `sudo npm install -g json-fmt`

MANDOS_SCEN_FILES=$(find . -name "*.scen.json")
MANDOS_STEP_FILES=$(find . -name "*.step.json")
MANDOS_STEPS_FILES=$(find . -name "*.steps.json")
MANDOS_ALL_FILES="$MANDOS_SCEN_FILES $MANDOS_STEP_FILES $MANDOS_STEPS_FILES"

TEMP_FILE=mandos-fmt-temp.scen.json
for MANDOS_FILE in $MANDOS_ALL_FILES
do
    echo $MANDOS_FILE
    json-fmt $MANDOS_FILE --indent "    " --prettify --output $TEMP_FILE || exit 1
    echo >> $TEMP_FILE # adds missing newline
    mv $TEMP_FILE $MANDOS_FILE
done

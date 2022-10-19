from pathlib import Path
from typing import List
import lldb
from lldb import SBDebugger, SBValue, SBFrame, SBBreakpointLocation


def __lldb_init_module(debugger: SBDebugger, dict):
    debugger.HandleCommand("breakpoint set --name breakpoint_marker_end_of_main")
    python_module_name = Path(__file__).with_suffix('').name
    breakpoint_function_name = end_of_main_breakpoint_handler.__name__
    debugger.HandleCommand(f"breakpoint command add -F {python_module_name}.{breakpoint_function_name}")


def get_string(rust_string: lldb.value) -> str:
    return rust_string.sbvalue.GetSummary()[1:-1] # remove surrounding quotes


def end_of_main_breakpoint_handler(frame: SBFrame, bp_loc: SBBreakpointLocation, internal_dict):
    print("TEST REPORT BEGIN")
    main_frame: SBFrame = frame.get_parent_frame()
    variables: List[SBValue] = main_frame.variables
    variable_summaries = {}
    for variable in variables:
        name = variable.GetName()
        if name == "to_check":
            to_check = variable
        summary = variable.GetSummary()
        variable_summaries[name] = summary
    correct = 0
    incorrect = 0
    duplicate = 0
    checked = set()
    for item in lldb.value(to_check):
        variable_name_string, value_to_check_string = item
        variable_name = get_string(variable_name_string)
        value_to_check = get_string(value_to_check_string)
        found = variable_summaries[variable_name]
        expected = value_to_check
        is_duplicate = variable_name in checked
        checked.add(variable_name)
        is_correct = found == expected
        if is_duplicate:
            duplicate += 1
            print(f"{variable_name} DUPLICATE")
        elif not is_correct:
            incorrect += 1
            print(f"{variable_name} MISMATCH: Found:\"{found}\", Expected:\"{expected}\"")
        else:
            correct += 1
            print(f"{variable_name} OK")
    if incorrect > 0 or duplicate > 0:
        test_result = "FAILED"
    else:
        test_result = "OK"
    print(f"Test {test_result} - correct ({correct}), incorrect ({incorrect}), duplicate({duplicate})")
    print("TEST REPORT END")

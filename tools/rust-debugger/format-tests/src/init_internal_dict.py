import sys

def __lldb_init_module(debugger, internal_dict):
    internal_dict['adapter_settings'] = {'sourceLanguages': ['rust'], 'evaluateForHovers': True, 'commandCompletions': True, 'displayFormat': 'auto', 'showDisassembly': 'auto', 'dereferencePointers': True, 'evaluationTimeout': 5.0, 'suppressMissingSourceFiles': True, 'consoleMode': 'commands', 'scriptConfig': {}, 'reproducer': False}

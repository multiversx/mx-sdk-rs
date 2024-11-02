import sys

def __lldb_init_module(debugger, internal_dict):
    internal_dict['adapter_settings'] = {'sourceLanguages': ['rust'], 'evaluateForHovers': True, 'commandCompletions': True, 'displayFormat': 'auto', 'showDisassembly': 'auto', 'dereferencePointers': True, 'evaluationTimeout': 5.0, 'suppressMissingSourceFiles': True, 'consoleMode': 'commands', 'scriptConfig': {}, 'reproducer': False}
    #sys.path.append('/home/alin/.vscode/extensions/vadimcn.vscode-lldb-1.11.0-linux-x64/adapter/scripts')
    sys.path.append('/home/alin/.vscode/extensions/vadimcn.vscode-lldb-1.11.0-linux-x64/lldb/lib/python3.9/site-packages')
    sys.path.append('/home/alin/.vscode/extensions/vadimcn.vscode-lldb-1.11.0-linux-x64/lldb/lib/python3.9/site-packages/setuptools-56.0.0-py3.9.egg')
    sys.path.append('/home/alin/.vscode/extensions/vadimcn.vscode-lldb-1.11.0-linux-x64/lldb/lib/python3.9/site-packages/pip-21.0.1-py3.9.egg')

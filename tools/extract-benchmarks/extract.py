#!/usr/bin/env python3
import contextlib
import re
import sys
from typing import Generator, List, Optional, Set, TextIO


class FunctionGasTrace:
    def __init__(self, function_name: str, total_gas_used: int, number_of_calls: int) -> None:
        self.function_name = function_name
        self.total_gas_used = total_gas_used
        self.number_of_calls = number_of_calls


class AddressReport:
    def __init__(self, contract_address: str) -> None:
        self.contract_address = contract_address
        self.function_gas_traces: List[FunctionGasTrace] = []

    def get_gas_trace_by_name(self, function_name: str) -> FunctionGasTrace:
        try:
            return next(trace for trace in self.function_gas_traces if trace.function_name == function_name)
        except StopIteration:
            return FunctionGasTrace(function_name, 0, 0)


class Transaction:
    def __init__(self, tx_id: str, step_type: str, function_name: str, total_gas_used: int) -> None:
        self.tx_id = tx_id
        self.step_type = step_type
        self.function_name = function_name
        self.total_gas_used = total_gas_used
        self.address_reports: List[AddressReport] = []

    def last_address_report(self) -> AddressReport:
        return self.address_reports[-1]


class Scenario:
    def __init__(self, scenario_name: str) -> None:
        self.scenario_name = scenario_name
        self.transactions: List[Transaction] = []

    def add_tx(self, transaction: Transaction) -> None:
        self.transactions.append(transaction)

    def normal_transactions(self) -> List[Transaction]:
        return [transaction for transaction in self.transactions if transaction.step_type != 'Deploy']

    def last_transaction(self) -> Transaction:
        return self.transactions[-1]


class Report:
    def __init__(self) -> None:
        self.scenarios: List[Scenario] = []

    def add_scenario(self, scenario_name: str) -> None:
        self.scenarios.append(Scenario(scenario_name))

    def last_scenario(self) -> Scenario:
        return self.scenarios[-1]

    def last_transaction(self) -> Transaction:
        return self.last_scenario().last_transaction()

    def last_report(self) -> AddressReport:
        return self.last_transaction().last_address_report()

    def add_tx(self, transaction: Transaction) -> None:
        self.last_scenario().add_tx(transaction)

    def add_address_report(self, address_report: AddressReport) -> None:
        self.last_transaction().address_reports.append(address_report)

    def add_gas_trace(self, function_gas_trace: FunctionGasTrace) -> None:
        self.last_report().function_gas_traces.append(function_gas_trace)


@contextlib.contextmanager
def output_redirect(output_file_name: Optional[str]) -> Generator[None, None, None]:
    if output_file_name:
        file = open(output_file_name, 'w')
        saved_stdout = sys.stdout
        sys.stdout = file
        try:
            yield
        finally:
            sys.stdout = saved_stdout
            file.close()
    else:
        yield


def print_summary(report: Report, output_file: Optional[str] = None) -> None:
    with output_redirect(output_file):
        for scenario in report.scenarios:
            print("Scenario, ", scenario.scenario_name)
            print()
            print("Function name, Total gas used")
            for transaction in scenario.normal_transactions():
                print(transaction.function_name, transaction.total_gas_used, sep=",")
            print()


def print_gas_traces(report: Report, output_file: Optional[str] = None) -> None:
    with output_redirect(output_file):
        function_names: Set[str] = set()
        for scenario in report.scenarios:
            for transaction in scenario.normal_transactions():
                for address_report in transaction.address_reports:
                    for function_gas_trace in address_report.function_gas_traces:
                        function_names.add(function_gas_trace.function_name)

        for scenario in report.scenarios:
            print("Scenario, ", scenario.scenario_name)
            print()
            print("Function name,Total gas used", *function_names, sep=",")
            for transaction in scenario.normal_transactions():
                gas_traces = [transaction.last_address_report().get_gas_trace_by_name(function_name).total_gas_used for function_name in function_names]
                print(transaction.function_name, transaction.total_gas_used, *gas_traces, sep=",")
            print()


def match_scenario(report: Report, line: str) -> None:
    found = re.match(r"Scenario: (.+) ... ", line)
    if found:
        scenario_name, = found.groups()
        report.add_scenario(scenario_name)


def match_tx(report: Report, line: str) -> None:
    found = re.match(r"In txID: (\w+) , step type:(\w+) ?,(?: function: (\w+) ?,)? total gas used: (\d+)", line)
    if found:
        try:
            tx_id, step_type, function_name, total_gas_used = found.groups()
        except ValueError:
            tx_id, step_type, total_gas_used = found.groups()
            function_name = ''
        report.add_tx(Transaction(tx_id, step_type, function_name, int(total_gas_used)))


def match_contract_address(report: Report, line: str) -> None:
    found = re.match(r"Gas Trace for:  SC Address (.+)", line)
    if found:
        contract_address, = found.groups()
        report.add_address_report(AddressReport(contract_address))


def match_gas_trace(report: Report, line: str) -> None:
    found = re.match(r"GasTrace: functionName: (\w+) ,  totalGasUsed: (\d+) , numberOfCalls: (\d+)", line)
    if found:
        function_name, total_gas_used, number_of_calls = found.groups()
        report.add_gas_trace(FunctionGasTrace(function_name, int(total_gas_used), int(number_of_calls)))


def extract(report_lines: List[str]) -> Report:
    report = Report()
    for line in report_lines:
        match_scenario(report, line)
        match_tx(report, line)
        match_contract_address(report, line)
        match_gas_trace(report, line)
    return report


def load_report(path: str) -> Report:
    with open(path, 'r') as file:
        return extract(file.readlines())


if __name__ == "__main__":
    report = load_report("bench.log")
    print_summary(report, "bench_summary.txt")
    print_gas_traces(report, "bench_detailed.txt")

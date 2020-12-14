import logging
from argparse import ArgumentParser
from pathlib import Path

from erdpy import config
from erdpy.accounts import Account
from erdpy.contracts import SmartContract
from erdpy.environments import TestnetEnvironment
from erdpy.projects import ProjectRust
from erdpy.proxy import ElrondProxy

logger = logging.getLogger("examples")

if __name__ == '__main__':
    parser = ArgumentParser()
    parser.add_argument("--proxy", help="Proxy URL", default=config.get_proxy())
    parser.add_argument("--contract", help="Existing contract address")
    parser.add_argument("--pem", help="PEM file", required=True)
    args = parser.parse_args()

    logging.basicConfig(level=logging.INFO)

    proxy = ElrondProxy(args.proxy)
    network = proxy.get_network_config()
    chain = network.chain_id
    gas_price = network.min_gas_price
    tx_version = network.min_tx_version

    environment = TestnetEnvironment(args.proxy)
    user = Account(pem_file=args.pem)

    project = ProjectRust(Path(__file__).parent.parent)
    bytecode = project.get_bytecode()

    # We initialize the smart contract with an actual address if IF was previously deployed,
    # so that we can start to interact with it ("query_flow")
    contract = SmartContract(address=args.contract)

    def deploy_flow():
        global contract

        # For deploy, we initialize the smart contract with the compiled bytecode
        contract = SmartContract(bytecode=bytecode)

        tx, address = environment.deploy_contract(
            contract=contract,
            owner=user,
            arguments=["0x0064"],
            gas_price=gas_price,
            gas_limit=50000000,
            value=None,
            chain=chain,
            version=tx_version
        )

        logger.info("Tx hash: %s", tx)
        logger.info("Contract address: %s", address.bech32())

    def get_sum_flow():
        answer = environment.query_contract(contract, "getSum")
        logger.info(f"Answer: {answer}")

    def add_flow(number):
        environment.execute_contract(
            contract=contract,
            caller=user,
            function="add",
            arguments=[number],
            gas_price=gas_price,
            gas_limit=50000000,
            value=None,
            chain=chain,
            version=tx_version
        )

    user.sync_nonce(ElrondProxy(args.proxy))

    while True:
        print("Let's run a flow.")
        print("1. Deploy")
        print("2. Query getSum()")
        print("3. Add()")

        try:
            choice = int(input("Choose:\n"))
        except Exception:
            break

        if choice == 1:
            environment.run_flow(deploy_flow)
            user.nonce += 1
        elif choice == 2:
            environment.run_flow(get_sum_flow)
        elif choice == 3:
            number = int(input("Enter number:"))
            environment.run_flow(lambda: add_flow(number))
            user.nonce += 1

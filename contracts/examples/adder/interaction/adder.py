import base64
import logging
from argparse import ArgumentParser

from erdpy import config
from erdpy.proxy import ElrondProxy
from erdpy.projects import ProjectRust
from erdpy.environments import TestnetEnvironment
from erdpy.contracts import SmartContract
from erdpy.accounts import Account

logger = logging.getLogger("examples")

if __name__ == '__main__':
    parser = ArgumentParser()
    parser.add_argument("--proxy", help="Testnet Proxy URL", default=config.get_proxy())
    parser.add_argument("--contract", help="Existing contract address")
    parser.add_argument("--pem", help="PEM file", required=True)
    args = parser.parse_args()

    logging.basicConfig(level=logging.INFO)

    project = ProjectRust("../")
    bytecode = project.get_bytecode()

    environment = TestnetEnvironment(args.proxy)
    user = Account(pem_file=args.pem)
    
    # We initialize the smart contract with an actual address if IF was previously deployed,
    # so that we can start to interact with it ("query_flow")
    contract = SmartContract(address=args.contract)

    def deploy_then_add_flow():
        global contract

        # For deploy, we initialize the smart contract with the compiled bytecode
        contract = SmartContract(bytecode=bytecode)
        user.sync_nonce(ElrondProxy(args.proxy))

        tx, address = environment.deploy_contract(
            contract=contract,
            owner=user,
            arguments=["0x0064"],
            gas_price=config.DEFAULT_GAS_PRICE,
            gas_limit=50000000,
            value=None,
            chain=config.get_chain_id(),
            version=config.get_tx_version()
        )

        logger.info("Tx hash: %s", tx)
        logger.info("Contract address: %s", address.bech32())

        # We increment our copy of the nonce
        user.nonce += 1
        environment.execute_contract(
            contract=contract, 
            caller=user,
            function="add",
            arguments=["0x0064"],
            gas_price=config.DEFAULT_GAS_PRICE,
            gas_limit=50000000,
            value=None,
            chain=config.get_chain_id(),
            version=config.get_tx_version()
        )

    def get_sum_flow():
        answer = environment.query_contract(contract, "getSum")
        logger.info(f"Answer: {answer}")

    while True:
        print("Let's run a flow.")
        print("1. Deploy")
        print("2. Query getSum()")

        try:
            choice = int(input("Choose:\n"))
        except Exception:
            break

        if choice == 1:
            environment.run_flow(deploy_then_add_flow)
        elif choice == 2:
            environment.run_flow(get_sum_flow)

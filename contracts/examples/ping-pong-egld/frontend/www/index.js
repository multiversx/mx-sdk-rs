import { tx_request, query_request } from "../pkg";

const pingButton = document.getElementById('pingButton');
const deployButton = document.getElementById('deployButton');
const pongButton = document.getElementById('pongButton');
const timestampButton = document.getElementById('timestampButton');
const deadlineButton = document.getElementById('deadlineButton');
const maxFundsButton = document.getElementById('getMaxFundsButton');
const userAddressesButton = document.getElementById('getUserAddresses');
const pingAmountButton = document.getElementById('getPingAmountButton');

const alice_wallet = "erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th";

let deployed_contract_address = "";


async function handlePing(event) {
    event.preventDefault();

    const egldValue = document.getElementById("addInput").value.trim();
    if (!egldValue) {
        alert('No value entered');
        return;
    }

    if (isNaN(egldValue)) {
        alert('Please enter a valid number');
        return;
    }

    if (Number(egldValue) < 0) {
        alert('Please enter a positive number');
        return;
    }

    if (egldValue.includes('.') && egldValue.split('.')[1].length > 18) {
        alert('Maximum 18 decimals allowed for EGLD amount');
        return;
    }

    console.log(`Entered value: ${egldValue} EGLD`);

    let body = {value: Number(egldValue), sender: alice_wallet, contract_address: deployed_contract_address};
    console.log(body);
    try {
        let res = await tx_request("ping", JSON.stringify(body));
        console.log(`Response: ${res.response}`);
    } catch (error) {
        console.error("Error:", error);
        alert("An error occurred while processing your request.");
    } finally {
        document.getElementById("addInput").value = '';
    }
}

async function handleDeploy(event) {
    event.preventDefault();

    let body = {
            ping_amount: 0.001, 
            max_funds: 100, 
            activation_timestamp: 'None',
            duration: 60,
            deployer: alice_wallet
        };
    
    console.log(body);

    try {
        let res = await tx_request("deploy", JSON.stringify(body));
        console.log(`Response: ${res.response}`);
        console.log(`Contract address: ${res.address}`);
        deployed_contract_address = res.address;
    } catch (error) {
        console.error("Error:", error);
        alert("An error occurred while processing your request.");
    }
}

async function handleTimestamp(event) {
    event.preventDefault();
    try {
        let res = await query_request("timestamp");
        console.log("Response:", res);
        alert(`Response: ${JSON.stringify(res)}`);   
    } catch (error) {
        console.error("Error:", error);
        alert("An error occurred while processing your request.");
    }
}


async function handlePong(event) {
    event.preventDefault();

    let body = {sender: alice_wallet, contract_address: deployed_contract_address};
    console.log(body);

    try {
        let res = await tx_request("pong", JSON.stringify(body));
        console.log(`Response: ${res.response}`);
    } catch (error) {
        console.error("Error:", error);
        alert("An error occurred while processing your request.");
    }
}

async function handleDeadline(event) {
    event.preventDefault();
    try {
        let res = await query_request("deadline");;
        console.log("Response:", res);
        alert(`Response: ${JSON.stringify(res)}`);   
    } catch (error) {
        console.error("Error:", error);
        alert("An error occurred while processing your request.");
    }
}

async function handleMaxFunds(event) {
    event.preventDefault();
    try {
        let res = await query_request("max_funds");
        console.log("Response:", res);
        alert(`Response: ${JSON.stringify(res)}`);   
    } catch (error) {
        console.error("Error:", error);
        alert("An error occurred while processing your request.");
    }
}

async function handlePingAmount(event) {
    event.preventDefault();
    try {
        let res = await query_request("ping_amount");
        console.log("Response:", res);
        alert(`Response: ${JSON.stringify(res)}`);   
    } catch (error) {
        console.error("Error:", error);
        alert("An error occurred while processing your request.");
    }
}

async function handleUserAddresses(event) {
    event.preventDefault();
    try {
        let res = await query_request("user_addresses");
        console.log("Response:", res);
        alert(`Response: ${JSON.stringify(res)}`);   
    } catch (error) {
        console.error("Error:", error);
        alert("An error occurred while processing your request.");
    }
}

pingButton.addEventListener('click', handlePing);
deployButton.addEventListener('click', handleDeploy);
pongButton.addEventListener('click', handlePong);
timestampButton.addEventListener('click', handleTimestamp);
deadlineButton.addEventListener('click', handleDeadline);
pingAmountButton.addEventListener('click', handlePingAmount);
maxFundsButton.addEventListener('click', handleMaxFunds);
userAddressesButton.addEventListener('click', handleUserAddresses);

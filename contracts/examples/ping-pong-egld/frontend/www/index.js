import { tx_request, query_request } from "../pkg";

const pingButton = document.getElementById('pingButton');
const deployButton = document.getElementById('deployButton');
const pongButton = document.getElementById('pongButton');
const deploySubmitButton = document.getElementById('deploySubmit');
const pingSubmitButton = document.getElementById('pingSubmit');
const pongSubmitButton = document.getElementById('pongSubmit');
const timestampButton = document.getElementById('timestampButton');
const deadlineButton = document.getElementById('deadlineButton');
const maxFundsButton = document.getElementById('getMaxFundsButton');
const userAddressesButton = document.getElementById('getUserAddresses');
const pingAmountButton = document.getElementById('getPingAmountButton');

// const alice_wallet = "erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th";

const deployModal = document.getElementById("deployModal");
const closeModalDeploy = document.getElementsByClassName("close")[0];
const closeModalPing = document.getElementsByClassName("close")[1];
const closeModalPong = document.getElementsByClassName("close")[2];
const pingModal = document.getElementById("pingModal");
const pongModal = document.getElementById("pongModal");
const bodyWrapper = document.getElementById("bodyWrapper");

function checkValidAmount (egldValue) {
    if (isNaN(egldValue)) {
        alert('Please enter a valid number');
        return false;
    }

    if (Number(egldValue) < 0) {
        alert('Please enter a positive number');
        return false;
    }

    if (egldValue.includes('.') && egldValue.split('.')[1].length > 18) {
        alert('Maximum 18 decimals allowed for EGLD amount');
        return false;
    }

    return true;
}

closeModalDeploy.onclick = function() {
    deployModal.classList.remove("show");
    document.getElementById('deployForm').reset();
    bodyWrapper.classList.remove("modal-open"); 
};

closeModalPing.onclick = function() {
    pingModal.classList.remove("show");
    document.getElementById('pingForm').reset();
    bodyWrapper.classList.remove("modal-open"); 
}

closeModalPong.onclick = function() {
    pongModal.classList.remove("show");
    document.getElementById('pongForm').reset();
    bodyWrapper.classList.remove("modal-open");
}

async function handlePing(event) {
    event.preventDefault();
    pingModal.classList.add("show");
    bodyWrapper.classList.add("modal-open");
}

async function handlePingSubmit(event) {
    event.preventDefault();
    const egldValue = document.getElementById("egldValue").value;
    const wallet_addr = document.getElementById("senderPing").value.trim();
    const contract_address = document.getElementById("contractAddrPing").value.trim();


    if (!egldValue) {
        alert("Please fill in all the fields.");
        return;
    }

    if (!checkValidAmount(egldValue))
        return;

    let body = {
        value: Number(egldValue),
        sender: wallet_addr,
        contract_address: contract_address
    };

    try {
        let res = await tx_request("ping", JSON.stringify(body));
        console.log(`Response: ${res.response}`);
    } catch (error) {
        console.error("Error:", error);
        alert("An error occurred while processing your request.");
    } finally {
        pingModal.style.display = "none";
        document.getElementById('pingForm').reset();
        bodyWrapper.classList.remove("modal-open");
    }
}

async function handleDeploy(event) {
    event.preventDefault();
    deployModal.classList.add("show");
    bodyWrapper.classList.add("modal-open");
}

async function handleDeploySumbit(event) {
    event.preventDefault();

    const pingAmount = document.getElementById("pingAmount").value;
    const maxFunds = document.getElementById("maxFunds").value;
    const activationTimestamp = document.getElementById("activationTimestamp").value || 'None';
    const duration = document.getElementById("duration").value;
    const deployer = document.getElementById("deployer").value;

    if (!pingAmount || !maxFunds || !activationTimestamp || !duration || !deployer) {
        alert("Please fill in all the fields.");
        return;
    }

    if (!checkValidAmount(pingAmount) || !checkValidAmount(maxFunds))
        return;

    let body = {
        ping_amount: Number(pingAmount),
        max_funds: Number(maxFunds),
        activation_timestamp: activationTimestamp,
        duration: Number(duration),
        deployer: deployer
    };

    console.log(body);

    try {
        let res = await tx_request("deploy", JSON.stringify(body));
        console.log(`Response: ${res.response}`);
        console.log(`Contract address: ${res.address}`);
    } catch (error) {
        console.error("Error:", error);
        alert("An error occurred while processing your request.");
    } finally {
        deployModal.classList.remove("show");
        document.getElementById('deployForm').reset();
        bodyWrapper.classList.remove("modal-open");
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

    pongModal.classList.add("show");
    bodyWrapper.classList.add("modal-open");
}

async function handlePongSubmit(event) {
    event.preventDefault();
    const wallet_addr = document.getElementById("senderPong").value.trim();
    const contract_address = document.getElementById("contractAddrPong").value.trim();

    let body = {
        sender: wallet_addr,
        contract_address: contract_address

    };
    console.log(body);

    try {
        let res = await tx_request("pong", JSON.stringify(body));
        console.log(`Response: ${res.response}`);
    } catch (error) {
        console.error("Error:", error);
        alert("An error occurred while processing your request.");
    } finally {
        pongModal.classList.remove("show");
        document.getElementById('pongForm').reset();
        bodyWrapper.classList.remove("modal-open");
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
    } finally {
        pongModal.classList.remove("show");
        document.getElementById('pongForm').reset();
        bodyWrapper.classList.remove("modal-open");
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

pingButton.addEventListener('click', handlePing);
deployButton.addEventListener('click', handleDeploy);
pongButton.addEventListener('click', handlePong);
deploySubmitButton.addEventListener('click', handleDeploySumbit);
pingSubmitButton.addEventListener('click', handlePingSubmit);
pongSubmitButton.addEventListener('click', handlePongSubmit);

timestampButton.addEventListener('click', handleTimestamp);
deadlineButton.addEventListener('click', handleDeadline);
pingAmountButton.addEventListener('click', handlePingAmount);
maxFundsButton.addEventListener('click', handleMaxFunds);
userAddressesButton.addEventListener('click', handleUserAddresses);

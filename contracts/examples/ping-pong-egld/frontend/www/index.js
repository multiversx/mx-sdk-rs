import { tx_request, query_request } from "../pkg";
import store, { clearScAddress, setScAddress } from './store';

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
const pingModal = document.getElementById("pingModal");
const pongModal = document.getElementById("pongModal");
const txStatusModal = document.getElementById("txStatusModal");
const timestampModal = document.getElementById("timestampModal");
const deadlineModal = document.getElementById("deadlineModal");
const maxFundsModal = document.getElementById("maxFundsModal");
const pingAmountModal = document.getElementById("pingAmountModal");
const userAddressesModal = document.getElementById("userAddressesModal");
const bodyWrapper = document.getElementById("bodyWrapper");

const closeModalDeploy = document.getElementsByClassName("close")[0];
const closeModalPing = document.getElementsByClassName("close")[1];
const closeModalPong = document.getElementsByClassName("close")[2];
const closeModalTxStatus = document.getElementsByClassName("close")[3];
const closeModalTimestamp = timestampModal.querySelector(".close");
const closeModalDeadline = deadlineModal.querySelector(".close");
const closeModalMaxFunds = maxFundsModal.querySelector(".close");
const closeModalPingAmount = pingAmountModal.querySelector(".close");
const closeModalUserAddresses = userAddressesModal.querySelector(".close");

const copyButton = document.getElementById("copyAddr");
const copySuccess = document.getElementById("copySuccess");
const copyIcon = document.getElementById("copyIcon");

const toggleButton = document.getElementById("toggleButton");
const contractModal = document.getElementById("contractAddrModal");
const scAddressContainer = document.getElementById("scAddressContainer");
const arrowDown = document.getElementById("arrowDown");
const arrowUp = document.getElementById("arrowUp");
const scAddress = document.getElementById("scAddress");
const redirectLink = document.getElementById('redirectLink');

async function getScAddress() {
    const cachedAddress = store.getState().scAddress.contract_address;
    console.log("Cached address:", cachedAddress);

    if (cachedAddress != '') {
        scAddress.innerText = cachedAddress;
        updateExplorerLink(cachedAddress);
        return;
    }

    let response = await query_request("contract_address");
    let contract_address = response.contract_address;

    if (contract_address !== '') {
        store.dispatch(setScAddress(contract_address));
        scAddress.innerText = contract_address;
        updateExplorerLink(contract_address);
    } else {
        scAddress.innerText = "No contract deployed";
    }
}

function updateExplorerLink(contract_address) {
    if (contract_address && contract_address !== "No contract deployed") {
        redirectLink.href = `https://devnet-explorer.multiversx.com/accounts/${contract_address}`;
        redirectLink.style.display = 'block';
    } else {
        redirectLink.style.display = 'none';
    }
}

function showStatusModal(statusText, imgSrc) {
    document.getElementById('statusImage').src = imgSrc;
    document.getElementById('status').innerText = statusText;
    
    txStatusModal.classList.add("show");

    if (statusText !== "In progress...") {
        setTimeout(() => {
            txStatusModal.classList.remove("show");
            bodyWrapper.classList.remove("modal-open");
        }, 4000);
    }
}

function showTimestampModal(timestampText) {
    document.getElementById('timestampResponse').innerText = timestampText;
    timestampModal.classList.add("show");
}

function showDeadlineModal(deadlineText) {
    document.getElementById('deadlineResponse').innerText = deadlineText;
    deadlineModal.classList.add("show");
}

function showMaxFundsModal(maxFundsText) {
    document.getElementById('maxFundsResponse').innerText = maxFundsText;
    maxFundsModal.classList.add("show");
}

function showPingAmountModal(pingAmountText) {
    document.getElementById('pingAmountResponse').innerText = pingAmountText;
    pingAmountModal.classList.add("show");
}

function showUserAddressesModal(responseText) {
    let responseObject;

    try {
        responseObject = JSON.parse(responseText);
    } catch (error) {
        userAddressesResponse.innerHTML = `<p>${responseText}</p>`;
        document.getElementById('userAddressesModal').classList.add('show');
        return;
    }

    if (responseObject.response && Array.isArray(responseObject.response)) {
        const addressesHtml = responseObject.response.map(address => `<li>${address}</li>`).join("");
        userAddressesResponse.innerHTML = `<ul>${addressesHtml}</ul>`;
        document.getElementById('userAddressesModal').classList.add('show');
    } else {
        userAddressesResponse.innerHTML = "<p>No addresses found.</p>";
    }
}

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

closeModalTimestamp.onclick = function () {
    timestampModal.classList.remove("show");
    bodyWrapper.classList.remove("modal-open");
};

closeModalDeadline.onclick = function () {
    deadlineModal.classList.remove("show");
    bodyWrapper.classList.remove("modal-open");
};

closeModalMaxFunds.onclick = function () {
    maxFundsModal.classList.remove("show");
    bodyWrapper.classList.remove("modal-open");
};

closeModalPingAmount.onclick = function () {
    pingAmountModal.classList.remove("show");
    bodyWrapper.classList.remove("modal-open");
};

closeModalUserAddresses.onclick = function () {
    userAddressesModal.classList.remove("show");
    bodyWrapper.classList.remove("modal-open");
};

closeModalTxStatus.onclick = function() {
    txStatusModal.classList.remove("show");
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

    pingModal.style.display = "none";
    document.getElementById('pingForm').reset();
    bodyWrapper.classList.remove("modal-open");

    try {
        showStatusModal("In progress...", "https://imgur.com/AmoFMD5.gif");
        let res = await tx_request("ping", JSON.stringify(body));
        showStatusModal("Success", "https://imgur.com/MfsVKLh.png");
        console.log(`Response: ${res.response}`);
    } catch (error) {
        console.error("Error:", error);
        showStatusModal("Error", "https://imgur.com/lVCaXg2.png")
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

    deployModal.classList.remove("show");
    document.getElementById('deployForm').reset();
    bodyWrapper.classList.remove("modal-open");

    try {
        showStatusModal("In progress...", "https://imgur.com/AmoFMD5.gif");
        let res = await tx_request("deploy", JSON.stringify(body));

        store.dispatch(clearScAddress());

        showStatusModal("Success", "https://imgur.com/MfsVKLh.png");
        await getScAddress();
    } catch (error) {
        console.error("Error:", error);
        showStatusModal("Error", "https://imgur.com/lVCaXg2.png")
    }
}

async function handleTimestamp(event) {
    event.preventDefault();

    try {
        let res = await query_request("timestamp");
        console.log("Response:", res);
        showTimestampModal(`Timestamp Response: ${JSON.stringify(res)}`);
    } catch (error) {
        console.error("Error:", error);
        if (error.message.includes('Failed to fetch')) {
            showTimestampModal(`Please deploy a contract first.`);
        } else {
            alert("An error occurred while processing your request.");
        }
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

    pongModal.classList.remove("show");
    document.getElementById('pongForm').reset();
    bodyWrapper.classList.remove("modal-open");

    try {
        showStatusModal("In progress...", "https://imgur.com/AmoFMD5.gif");
        let res = await tx_request("pong", JSON.stringify(body));
        showStatusModal("Success", "https://imgur.com/MfsVKLh.png");
        console.log(`Response: ${res.response}`);
    } catch (error) {
        console.error("Error:", error);
        showStatusModal("Error", "https://imgur.com/lVCaXg2.png")
    }
}

async function handleDeadline(event) {
    event.preventDefault();
    try {
        let res = await query_request("deadline");;
        console.log("Response:", res);
        showDeadlineModal(`Deadline Response: ${JSON.stringify(res)}`); 
    } catch (error) {
        console.error("Error:", error);
        if (error.message.includes('Failed to fetch')) {
            showDeadlineModal(`Please deploy a contract first.`); 
        } else {
            alert("An error occurred while processing your request.");
        }
    }
}

async function handlePingAmount(event) {
    event.preventDefault();
    try {
        let res = await query_request("ping_amount");
        console.log("Response:", res);
        showPingAmountModal(`Ping Amount Response: ${JSON.stringify(res)}`); 
    } catch (error) {
        console.error("Error:", error);
        if (error.message.includes('Failed to fetch')) {
            showPingAmountModal(`Please deploy a contract first.`);
        } else {
            alert("An error occurred while processing your request.");
        }
    }
}

async function handleUserAddresses(event) {
    event.preventDefault();
    try {
        let res = await query_request("user_addresses");
        console.log("Response:", res);
        showUserAddressesModal(JSON.stringify(res));
    } catch (error) {
        console.error("Error:", error);
        if (error.message.includes('Failed to fetch')) {
            showUserAddressesModal(`Please deploy a contract first.`);
        } else {
            alert("An error occurred while processing your request.");
        }
    }
}

async function handleMaxFunds(event) {
    event.preventDefault();
    try {
        let res = await query_request("max_funds");
        console.log("Response:", res);
        showMaxFundsModal(`Max Funds Response: ${JSON.stringify(res)}`);
    } catch (error) {
        console.error("Error:", error);
        if (error.message.includes('Failed to fetch')) {
            showMaxFundsModal(`Please deploy a contract first.`);
        } else {
            alert("An error occurred while processing your request.");
        }
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

toggleButton.addEventListener("click", () => {
  const isExpanded = contractModal.classList.contains("expanded");

  if (isExpanded) {
    contractModal.classList.remove("expanded");
    contractModal.classList.add("compressed");
    arrowUp.style.display = "none";
    arrowDown.style.display = "block";
    scAddressContainer.style.display = "none";
  } else {
    contractModal.classList.remove("compressed");
    contractModal.classList.add("expanded");
    arrowUp.style.display = "block";
    arrowDown.style.display = "none";
    scAddressContainer.style.display = "flex";
  }
});

copyButton.addEventListener('click', () => {
    let contract_address = document.getElementById("scAddress").innerText;

    if (contract_address === 'No contract deployed') {
        return
    }
    navigator.clipboard.writeText(contract_address).then(() => {
        copyIcon.style.display = "none";
        copySuccess.style.display = "block";

        setTimeout(() => {
            copyIcon.style.display = "block";
            copySuccess.style.display = "none";
        }, 2000);
    }).catch((error) => {
        console.error("Error copying:", error);
    });
});


await getScAddress();

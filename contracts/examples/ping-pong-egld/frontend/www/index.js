import { ping } from "../pkg";

const pingButton = document.getElementById('pingButton');

async function handlePing(event) {
    event.preventDefault();

    const egldValue = document.getElementById("addInput").value.trim();
    if (!egldValue) {
        alert('No value entered');
        return;
    }

    console.log(`Entered value: ${egldValue} EGLD`);
    let body = {value: Number(egldValue) * Math.pow(10, 18)};
    console.log(body);
    try {
        let res = await ping(egldValue, JSON.stringify(body));
        console.log(`Response: ${res.response}`);
    } catch (error) {
        console.error("Error:", error);
        alert("An error occurred while processing your request.");
    } finally {
        document.getElementById("addInput").value = '';
    }
}

pingButton.addEventListener('click', handlePing);
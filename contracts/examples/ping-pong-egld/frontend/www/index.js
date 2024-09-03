import {greet} from "../pkg";
import './style.css';

const pingButton = document.getElementById('pingButton');

pingButton.addEventListener('click', function() {
    const egldValue = document.getElementById("addInput").value.trim(); // Trim to remove unnecessary spaces

    if (!egldValue) { 
        alert('No value entered');
        return;
    } 

    console.log(`Entered value: ${egldValue} EGLD`);
    greet(egldValue);

    // Clear the input after processing
    document.getElementById("addInput").value = '';
});


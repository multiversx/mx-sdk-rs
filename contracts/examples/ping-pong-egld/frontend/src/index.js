document.getElementById('pingButton').addEventListener('click', function() {
    const egldValue = document.getElementById("addInput").value;

    if (!egldValue) {
        alert('No value entered');
        return;
    } else {
        console.log(`Entered value: ${egldValue} EGLD`);
        alert('Transaction sent');
    }
    document.getElementById("addInput").value = '';
});

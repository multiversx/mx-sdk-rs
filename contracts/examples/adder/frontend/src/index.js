document.getElementById('addButton').addEventListener('click', function() {
    const inputValue = document.getElementById("addInput").value;

    if (!inputValue) {
        alert('No value entered');
        return;
    } else {
        console.log(`Entered value: ${inputValue}`);
        alert('Transaction sent');
    }
    document.getElementById("addInput").value = '';
});

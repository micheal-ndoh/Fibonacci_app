document.addEventListener('DOMContentLoaded', () => {
    const inputField = document.getElementById('fibonacci-input');
    const calculateButton = document.getElementById('calculate-button');
    const resultDisplay = document.getElementById('result-display');

    calculateButton.addEventListener('click', async () => {
        const inputValue = inputField.value;

        if (!isNaN(inputValue) && inputValue.trim() !== '') {
            const number = parseInt(inputValue);
            const result = await window.__TAURI__.invoke('calculate_fibonacci', { number });
            resultDisplay.textContent = `Fibonacci of (${number}) = ${result}`;
        } else {
            resultDisplay.textContent = 'Please enter a valid number.';
        }
    });
});
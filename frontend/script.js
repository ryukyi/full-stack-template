/**
 * Fetches meal data from the backend and updates the DOM to display the meals.
 * This function is called once the DOM content is fully loaded.
 */
document.addEventListener('DOMContentLoaded', async function() {
    try {
        await fetchMeals();
    } catch (error) {
        console.error('Error initializing meal fetch:', error);
    }
});

async function fetchMeals() {
    try {
        // Use async/await syntax for cleaner code
        const response = await fetch('api/meals'); // Adjust the URL/port as necessary
        const data = await response.json();

        const list = document.getElementById('meals-list');
        // Clear existing meals to prevent duplication
        list.innerHTML = '';

        // Iterate over each meal and append it to the list
        data.forEach(meal => {
            const item = document.createElement('li');
            item.textContent = `${meal.name}: ${meal.description} - $${meal.price}`;
            list.appendChild(item);
        });
    } catch (error) {
        // Enhanced error handling
        console.error('Error fetching meals:', error);
        // Optionally, update the UI to inform the user that an error occurred
    }
}
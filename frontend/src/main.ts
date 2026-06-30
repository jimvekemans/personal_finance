// frontend/src/main.ts
import './style.css'

document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
  <section id="expense-form-section">
    <h1>Add Expense</h1>
    <form id="expense-form">
      <div>
        <label for="item">Item</label>
        <input type="text" id="item" name="item" required />
      </div>
      <div>
        <label for="cost">Cost (€)</label>
        <input type="number" id="cost" name="cost" step="0.01" required />
      </div>
      <div>
        <label for="category">Category</label>
        <select id="category" name="category" required>
          <option value="fixed">Fixed</option>
          <option value="groceries">Groceries</option>
          <option value="personal">Personal</option>
          <option value="other">Other</option>
        </select>
      </div>
      <div>
        <label for="who_paid">Who Paid</label>
        <select id="who_paid" name="who_paid" required>
          <option value="Jim">Jim</option>
          <option value="Zina">Zina</option>
        </select>
      </div>
      <button type="submit">Submit Expense</button>
    </form>
    <div id="status-message"></div>
  </section>
`;

const form = document.querySelector<HTMLFormElement>('#expense-form')!;
const statusMessage = document.querySelector<HTMLDivElement>('#status-message')!;

form.addEventListener('submit', async (e) => {
  e.preventDefault();
  const formData = new FormData(form);
  const data = {
    item: formData.get('item'),
    cost: parseFloat(formData.get('cost') as string),
    category: formData.get('category'),
    who_paid: formData.get('who_paid')
  };

  try {
    const response = await fetch('/api/expenses', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(data)
    });

    if (response.ok) {
      statusMessage.textContent = 'Expense added successfully!';
      statusMessage.style.color = 'green';
      form.reset();
    } else {
      statusMessage.textContent = 'Failed to add expense.';
      statusMessage.style.color = 'red';
    }
  } catch (error) {
    statusMessage.textContent = 'Network error occurred.';
    statusMessage.style.color = 'red';
  }
});
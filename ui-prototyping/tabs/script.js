const searchField = document.getElementById('search-input');
const searchResults = document.getElementById('container_search-results');

const data = {
  1: {
    body: 'apple',
  },
  2: {
    body: 'banana',
  }
};

const searchResult = (str, id) => `<div id="${id}" class="search-result">
  ${str}
</div>`;

const appendedItems = [];

searchField.addEventListener('keyup', (e) => {
  searchResults.innerHTML = '';

  if (e.target.value.length) {
    searchResults.innerHTML += searchResult('item', 1);
    searchResults.innerHTML += searchResult('item', 2);
  }
});

// bind tab clicks
document.addEventListener('click', (e) => {
  if (Array.from(e.target.classList).indexOf('search-result') !== -1) {
    alert('clicked');
  }
});
let lastCheckboxToggled = null;

function toggleSelectRecipes(buttonEl) {
  const isPressed = buttonEl.ariaPressed === "false";
  if (!isPressed) {
    lastCheckboxToggled = null;
    document.querySelector("#num-recipes-selected > span").textContent = 0;
  }

  buttonEl.ariaPressed = isPressed;
  document.getElementById("list-recipes-wrapper").dataset.select = isPressed;
  document.querySelectorAll(".checkbox-select").forEach((checkbox) => (checkbox.checked = false));
  currentMode.select = isPressed;
}

function handleSelectRecipe(checkbox, { shiftKey }) {
  const checkboxes = [...document.querySelectorAll("#list-recipes section input[type=checkbox]")];

  if (shiftKey && lastCheckboxToggled) {
    let [start, end] = [checkboxes.indexOf(lastCheckboxToggled), checkboxes.indexOf(checkbox)].sort((a, b) => a - b);
    checkboxes.slice(start, end + 1).forEach((cb) => (cb.checked = lastCheckboxToggled.checked));
  }

  lastCheckboxToggled = checkbox;
  const numChecked = checkboxes.filter((cb) => cb.checked).length;
  document.querySelector("#num-recipes-selected > span").textContent = numChecked;
  document.querySelectorAll("#select-recipes-actions button").forEach((button) => (button.disabled = numChecked === 0));
}

function selectAllRecipesInView() {
  const checkboxes = [...document.querySelectorAll("#list-recipes section input[type=checkbox]")];
  checkboxes.forEach((checkbox) => (checkbox.checked = true));

  const numChecked = checkboxes.length;
  document.querySelector("#num-recipes-selected > span").textContent = numChecked;
  document.querySelectorAll("#select-recipes-actions button").forEach((button) => (button.disabled = false));
}

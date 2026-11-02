document.addEventListener("DOMContentLoaded", function () {
  initGlobalKeyboardShortcuts();
  syncLayout();
  window.todayISO = () => new Date().toISOString().split("T")[0];
  window.currentTime = () => new Date().getTime();
});

function initGlobalKeyboardShortcuts() {
  document.addEventListener("keydown", (event) => {
    const key = event.key.toLowerCase();
    const isRecipesPage = window.location.pathname === "/recipes";
    const isViewRecipePage = window.location.pathname.match(/^\/recipes\/(\d+)$/);

    if (event.ctrlKey || event.metaKey) {
      if (event.altKey) {
        switch (key) {
          case "d":
            if (isViewRecipePage) {
              htmx.trigger("#duplicate-recipe", "click");
            }
            break;
          case "i":
            htmx
              .ajax("GET", "/recipes/add", {
                target: "#content",
              })
              .then(() => {
                document.querySelector("#import-recipes-dialog").showModal();
              })
              .then(() => {
                window.history.pushState({}, "", "/recipes/add");
              });
            break;
          case "n":
            htmx
              .ajax("GET", "/recipes/add/manual", {
                target: "#content",
              })
              .then(() => {
                window.history.pushState({}, "", "/recipes/add/manual");
              });
            break;
          case "s":
            if (
              window.location.pathname === "/recipes/add/manual" ||
              window.location.pathname.match(/^\/recipes\/(\d+)\/edit$/)
            ) {
              const form = document.querySelector("form.card-body");
              if (form) {
                form.requestSubmit();
              }
            } else {
              htmx
                .ajax("GET", "/settings", {
                  target: "#settings-dialog-content",
                })
                .then(() => {
                  document.querySelector("#settings-dialog").showModal();
                });
            }
            break;
          case "r":
            htmx
              .ajax("GET", "/reports", {
                target: "#content",
              })
              .then(() => {
                window.history.pushState({}, "", "/reports");
              });
            break;
          case "w":
            htmx
              .ajax("GET", "/recipes/add", {
                target: "#content",
              })
              .then(() => {
                document.querySelector("#websites-dialog").showModal();
                window.history.pushState({}, "", "/recipes/add");
              });
            break;
          default:
            break;
        }
      } else if (event.shiftKey) {
        switch (key) {
          case "f":
            if (isViewRecipePage) {
              const id = parseInt(isViewRecipePage[1], 10);
              htmx.trigger(`#favourite-${id}`, "click");
            }
            break;
          default:
            break;
        }
      } else {
        switch (key) {
          case "a":
            if (isRecipesPage && currentMode.select) {
              event.preventDefault();
              selectAllRecipesInView();
            }
            break;
          case "e":
            if (isViewRecipePage) {
              htmx.trigger("#edit-recipe", "click");
            }
            break;
          case "s":
            if (
              window.location.pathname === "/recipes/add/manual" ||
              window.location.pathname.match(/^\/recipes\/(\d+)\/edit$/)
            ) {
              event.preventDefault();
              event.stopPropagation();

              const form = document.querySelector("form.card-body");
              if (form) {
                form.requestSubmit();
              }
            }
            break;
          case "p":
            if (window.location.pathname === "/shopping/lists") {
              event.preventDefault();
              document.getElementById("print-shopping-list-button").click();
            }
            break;
          case "x":
            if (isViewRecipePage) {
              const url = `/recipes/${parseInt(isViewRecipePage[1], 10)}/share`;
              htmx.ajax("POST", url, {
                target: "#content",
              });
            }
            break;
          default:
            break;
        }
      }
    } else if (key === "delete") {
      let viewRecipePage = window.location.pathname.match(/^\/recipes\/(\d+)$/);
      if (viewRecipePage) {
        if (confirm("Are you sure you want to delete this recipe?")) {
          fetch(`/recipes/${parseInt(viewRecipePage[1], 10)}`, {
            method: "DELETE",
          }).then(() => {
            window.location.replace("/recipes");
          });
        }
      }
    }
  });
}

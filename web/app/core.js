const cookbooksPattern = new RegExp("^/cookbooks/\\d+(/recipes/search.*)?$");
const cookbooksSharePattern = new RegExp("^/c/[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$");

const recipesPattern = new RegExp("^/recipes/\\d+(/edit)?$");
const recipesSharePattern = new RegExp("^/r/[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$");

const reportsPattern = new RegExp("^/reports(/\\d+)?$");

const pathsShowRecipesSidebar = [
    "/",
    "/cookbooks",
    "/recipes",
];

const pathsHideAddRecipeButton = [
    "/admin",
    "/cookbooks",
    "/recipes/add",
    "/recipes/add/manual",
];

function showAll() {
    showAddRecipeButton();
    showAddCookbookButton();
    showCookbookTitle();
    showRecipesSidebar();
}

function showAddRecipeButton() {
    const isRecipe = recipesPattern.test(location.pathname) || recipesSharePattern.test(location.pathname);
    const el = document.querySelector("#add_recipe");

    if (isRecipe ||
        pathsHideAddRecipeButton.some(path => path === location.pathname) ||
        cookbooksPattern.test(location.pathname) ||
        cookbooksSharePattern.test(location.pathname) ||
        reportsPattern.test(location.pathname)) {
        el?.classList.add("hidden");
    } else {
        el?.classList.remove("hidden");
    }
}

function showAddCookbookButton() {
    const el = document.querySelector("#add_cookbook");
    if (el) {
        add_cookbook.setAttribute("hx-target", "#content");
        add_cookbook.setAttribute("hx-swap", "innerHTML")
        htmx.process(add_cookbook);
    }

    if (location.pathname === "/cookbooks") {
        el?.classList.remove("hidden");
    } else {
        el?.classList.add("hidden");
    }
}

function showCookbookTitle() {
    const cookbookTitleDiv = document.querySelector("#content-title");
    if (cookbooksPattern.test(location.pathname) ||
        cookbooksSharePattern.test(location.pathname)) {
        cookbookTitleDiv?.classList.add("md:block");
    } else {
        cookbookTitleDiv?.classList.remove("md:block");
    }
}

function showRecipesSidebar() {
    const desktop = document.querySelector("#desktop_nav");
    const mobile = document.querySelector("#mobile_nav");

    if (pathsShowRecipesSidebar.includes(location.pathname) || cookbooksPattern.test(location.pathname)) {
        desktop?.firstElementChild.classList.remove("hidden");
        mobile?.classList.remove("hidden");
    } else {
        desktop?.firstElementChild.classList.add("hidden");
        mobile?.classList.add("hidden");
    }

    if (recipesPattern.test(location.pathname) || recipesSharePattern.test(location.pathname) || location.pathname === "/admin" || reportsPattern.test(location.pathname)) {
        desktop?.firstElementChild.classList.add("hidden");
        mobile?.classList.add("hidden");
    } else {
        desktop?.firstElementChild.classList.remove("hidden");
        mobile?.classList.remove("hidden");
    }
}

function loadSortableJS() {
    return loadScript("https://cdn.jsdelivr.net/npm/sortablejs@latest/Sortable.min.js")
}

function loadScript(url) {
    const script = document.createElement("script");
    script.src = url;
    document.body.appendChild(script);

    return new Promise((res, rej) => {
        script.onload = () => res();
        script.onerror = () => rej();
    });
}

function downloadFile(data, filename, mime) {
    const blobURL = window.URL.createObjectURL(data);
    const a = document.createElement('a');
    a.style.display = 'none';
    a.href = blobURL;
    a.setAttribute('download', filename);
    if (typeof a.download === 'undefined') {
        a.setAttribute('target', '_blank');
    }
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    setTimeout(() => {
        window.URL.revokeObjectURL(blobURL);
    }, 100);
}

window.addEventListener("DOMContentLoaded", () => {
    showAll();
    document.addEventListener("htmx:pushedIntoHistory", showAll);

    document.body.addEventListener("showMessageHtmx", function (event) {
        const {action, message, status, title} = event.detail;
        showToast(title, message, status, action);
    })
});

document.addEventListener("htmx:beforeProcessNode", () => {
    const el = document.querySelector("#add_cookbook");
    if (el) {
        if (document.querySelector(".cookbooks-display") === null) {
            add_cookbook.setAttribute("hx-target", "#content");
            add_cookbook.setAttribute("hx-swap", "innerHTML");
        } else {
            add_cookbook.setAttribute("hx-target", ".cookbooks-display");
            add_cookbook.setAttribute("hx-swap", "beforeend");

            const p = document.querySelector("#pagination");
            if (p && !p.querySelector("button:nth-last-child(2)").classList.contains('btn-active')) {
                add_cookbook.setAttribute("hx-swap", "none");
            }
        }
        htmx.process(add_cookbook);
    }
});

htmx.on('htmx:pushedIntoHistory', () => {
    showAll();
    document.addEventListener("htmx:pushedIntoHistory", showAll);
});

document.addEventListener("htmx:wsBeforeMessage", (event) => {
    try {
        const {type, data, fileName} = JSON.parse(event.detail.message);
        switch (type) {
            case "toast":
                const {title, message, background, action} = data;
                showToast(title, message, background, action);
                break;
            case "file":
                const decoded = atob(data);
                const bytes = new Uint8Array(decoded.length);
                for (let i = 0; i < decoded.length; i++) {
                    bytes[i] = decoded.charCodeAt(i);
                }
                const blob = new Blob([bytes], {type: "application/zip"});
                downloadFile(blob, fileName, "application/zip");
                event.preventDefault();
                break;
        }
    } catch (_) {
    }
});

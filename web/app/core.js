document.addEventListener('DOMContentLoaded', function () {
    initGlobalKeyboardShortcuts();
    syncLayout();
});

document.body.addEventListener('htmx:afterSwap', (event) => {
    if (event.target.id === 'content') {
        syncLayout();
    }
});

document.body.addEventListener('htmx:historyRestore', () => {
    syncLayout();
});

function loadSortableJS() {
    return loadScript("https://cdn.jsdelivr.net/npm/sortablejs@latest/Sortable.min.js")
}

function loadRecipesManualScripts() {
    loadScript("https://cdn.jsdelivr.net/npm/html-duration-picker@latest/dist/html-duration-picker.min.js")
        .then(() => HtmlDurationPicker.init())

    loadScript("https://cdn.jsdelivr.net/npm/sortablejs@latest/Sortable.min.js")
        .then(() => {
            const inputs = [
                {name: 'tool', type: 'input'},
                {name: 'ingredient', type: 'input'},
                {name: 'instruction', type: 'textarea'},
            ];
            inputs.forEach(({name, type}) => {
                const list = document.querySelector(`#${name}s-list`);
                if (list) {
                    new Sortable.create(list, {
                        handle: '.handle',
                        animation: 150,
                    });
                }
            });
        });
}

function addMedia(event) {
    let media = document.querySelectorAll("#media label");
    for (let i = 0; i < media.length; i++) {
        if (media[i].querySelector('input[type="file"]').files.length === 0) {
            alert(`Please select an image or video for 'Media ${i + 1}'`);
            return;
        }
    }

    const buttons = document.querySelectorAll(".buttons-container button");
    buttons.forEach(el => el.classList.remove("btn-active"));

    const cloneMedia = media[media.length - 1].cloneNode(true);
    media.forEach(label => label.classList.add("hidden"));
    cloneMedia.querySelector("input").value = '';
    cloneMedia.id = `media-${media.length + 1}`;

    let n = cloneMedia.querySelector("img");
    if (n) {
        n.src = '';
    } else {
        n = cloneMedia.querySelector("video");
        n.src = '';
    }

    cloneMedia.querySelector("div").classList.remove("hidden");
    cloneMedia.querySelector("input[type='url']").value = '';
    const subButtons = cloneMedia.querySelectorAll("button");
    subButtons[subButtons.length - 1].classList.add("hidden");
    document.querySelector("#media").appendChild(cloneMedia);
    const video = cloneMedia.querySelector('video');
    if (video) {
        n.classList.remove("hidden");
        video.parentNode.removeChild(video);
    }
    media = document.querySelectorAll("#media label");
    media[media.length - 1].classList.remove("hidden");
    _hyperscript.processNode(cloneMedia);

    let target = event.target;
    if (["svg", "circle"].includes(target.tagName)) {
        target = event.target.parentElement;
    }

    const cloneButton = target.previousElementSibling.cloneNode(true);
    cloneButton.id = `media-button-${buttons.length}`;
    cloneButton.classList.add("btn-active");
    cloneButton.textContent = `Media ${buttons.length}`;
    cloneButton.setAttribute("onclick", "switchMedia(event)");
    target.previousElementSibling.parentNode.insertBefore(cloneButton, target);
    _hyperscript.processNode(cloneButton);
    htmx.process(cloneMedia);
}

function deleteMedia(event) {
    let parent = event.target.parentElement;
    if (parent.tagName === "SPAN") {
        parent = parent.parentElement;
    }

    const img = parent.querySelector("img");
    img.src = "";
    img.value = "";

    if (img.nextElementSibling.tagName === "VIDEO") {
        img.parentElement.removeChild(img.nextElementSibling);
        img.classList.remove("hidden");
    }

    const span = parent.querySelector("span");
    span.querySelector("input[type=file]").value = null;
    span.querySelector("input[type=file]").files = (new DataTransfer()).files;
    span.children[0].classList.remove("hidden");
    event.target.classList.add("hidden");
}

function addKeyword(event) {
    const input = document.querySelector("#new-keyword")
    const keyword = input.value.trim();
    if (keyword === "") {
        return;
    }

    const div = document.querySelector("#hidden-keyword").cloneNode(true);
    div.classList.remove("hidden");
    div.querySelector("input").value = keyword;
    div.querySelector("span").textContent = keyword;
    _hyperscript.processNode(div);
    htmx.process(div);

    const container = document.querySelector("#empty-keyword");
    container.parentNode.insertBefore(div, container);

    input.value = '';
    input.focus();
}

function switchMedia(event) {
    let target = event.target;
    if (target.tagName === "svg") {
        target = event.target.parentElement;
    }

    const media = document.querySelectorAll("#media label");
    for (let i = 0; i < media.length; i++) {
        if (media[i].id === target.id.replace("button-", "")) {
            media.forEach(label => label.classList.add("hidden"));
            media[i].classList.remove("hidden");
            document.querySelectorAll(".buttons-container button").forEach(el => el.classList.remove("btn-active"));
            document.getElementById(event.target.id).classList.add("btn-active");
            return;
        }
    }
}

function isUrl(event) {
    let url;
    try {
        url = new URL(event.target.previousElementSibling.value);
    } catch (_) {
        return false;
    }
    return url.protocol === "http:" || url.protocol === "https:";
}

function updateMediaFromFetch(input, url) {
    if (url === window.location.href) {
        input.value = '';
        return;
    }

    const pathRegexImage = /^\/data\/images\/([\da-z]{8}-([\da-z]{4}-){3}[\da-z]{12}).webp$/;
    const pathRegexVideo = /^\/data\/videos\/([\da-z]{8}-([\da-z]{4}-){3}[\da-z]{12}).webm$/;
    const urlObject = new URL(url);

    let uuid;
    if (pathRegexImage.test(urlObject.pathname)) {
        uuid = urlObject.pathname.match(pathRegexImage)[1];
    } else if (pathRegexVideo.test(urlObject.pathname)) {
        uuid = urlObject.pathname.match(pathRegexVideo)[1];
    } else {
        uuid = 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function (c) {
            const r = Math.random() * 16 | 0;
            const v = c === 'x' ? r : (r & 0x3 | 0x8);
            return v.toString(16);
        });
    }
    fetch(url)
        .then(response => response.blob())
        .then(blob => {
            const file = new File([blob], uuid, {type: blob.type});
            const dataTransfer = new DataTransfer();
            dataTransfer.items.add(file);
            input.files = dataTransfer.files;
            input.dispatchEvent(new Event('change'));
        });
}

function pasteText(inputEl, values) {
    const ol = inputEl.closest('ol');
    const valueArray = values.split('\n').map(v => v.trim()).filter(v => v);

    if (!valueArray.length) {
        return;
    }

    let items = ol.querySelectorAll('li');
    let currentIndex = inputEl.value.trim() === ''
        ? Array.from(items).indexOf(inputEl.closest('li'))
        : items.findIndex(li => li.querySelector('input, textarea')?.value.trim() === '');

    if (currentIndex === -1) {
        currentIndex = 0;
    }

    valueArray.forEach((value, index) => {
        const targetIndex = currentIndex + index;

        if (targetIndex < items.length) {
            const input = items[targetIndex].querySelector('input, textarea');
            if (input) {
                input.value = value;
            }
        } else {
            const clone = ol.lastElementChild.cloneNode(true);
            const input = clone.querySelector('input, textarea');
            if (input) {
                input.value = value;
            }

            _hyperscript.processNode(clone);
            htmx.process(clone);

            ol.appendChild(clone);
            items = ol.querySelectorAll('li');
        }
    });

    const lastIndex = currentIndex + valueArray.length - 1;
    if (lastIndex < items.length) {
        const lastInput = items[lastIndex].querySelector('input, textarea');
        if (lastInput) {
            setTimeout(() => {
                lastInput.value = valueArray[valueArray.length - 1];
                lastInput.dispatchEvent(new Event('input', {bubbles: true}));
                lastInput.blur();
            }, 10);
            lastInput.focus();
        }
    }
}

function addItem(event, isPastedText = false) {
    const ol = event.target.closest('ol');
    const items = ol.querySelectorAll('li');

    for (let i = 0; i < items.length; i++) {
        let input = items[i].querySelector('input');
        if (!input) {
            input = items[i].querySelector('textarea');
        }

        if (input.value === '') {
            return;
        }
    }

    const clone = event.target.closest('li').cloneNode(true);
    let el = 'input';

    try {
        clone.querySelector(el).value = '';
    } catch {
        el = 'textarea';
        clone.querySelector(el).value = '';
    }

    _hyperscript.processNode(clone);
    htmx.process(clone);
    ol.appendChild(clone);

    clone.querySelector(el).focus();
}

async function pasteImage(event) {
    try {
        const clipboardItems = await navigator.clipboard.read();
        clipboardItems.forEach(async (item) => {
            const uuid = 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function (c) {
                const r = Math.random() * 16 | 0;
                const v = c === 'x' ? r : (r & 0x3 | 0x8);
                return v.toString(16);
            });

            const blob = await item.getType(item.types.find(t => t.startsWith("image/")));
            const file = new File([blob], uuid, {type: blob.type});

            const dataTransfer = new DataTransfer();
            dataTransfer.items.add(file);

            const input = event.target.parentElement.parentElement.querySelector("input[type=file]");
            input.files = dataTransfer.files;
            input.dispatchEvent(new Event('change'));
        })
    } catch (err) {
        console.error(err.name, err.message);
        alert("No image in clipboard or ould not paste image.");
    }
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
    document.body.addEventListener("htmx:wsAfterMessage", function (event) {
        try {
            const data = event.detail.message;
            const parsed = typeof data === "string" ? JSON.parse(data) : data;

            if (parsed.showMessageHtmx) {
                const {action, message, status, title} = parsed.showMessageHtmx;
                showToast(title, message, status, action);
            }
        } catch (err) {
            console.error(`Failed to parse WebSocket message: ${err}`);
        }
    })
});

document.addEventListener("htmx:beforeProcessNode", () => {
    const el = document.querySelector("#add-cookbook");
    if (el) {
        if (document.querySelector(".cookbooks-display") === null) {
            el.setAttribute("hx-target", "#content");
            el.setAttribute("hx-swap", "innerHTML");
        } else {
            el.setAttribute("hx-target", ".cookbooks-display");
            el.setAttribute("hx-swap", "beforeend");

            const p = document.querySelector("#pagination");
            if (p && !p.querySelector("button:nth-last-child(2)").classList.contains('btn-active')) {
                el.setAttribute("hx-swap", "none");
            }
        }
        htmx.process(el);
    }
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

function updateAddCookbookUrl(selectedPage) {
    const el = document.querySelector("#add-cookbook");
    if (el) {
        el.setAttribute("hx-post", `/cookbooks?page=${selectedPage}`);
        htmx.process(el);
    }
}

function copyToClipboard(text) {
    if (window.navigator.clipboard) {
        navigator.clipboard.writeText(text).then(() => {
        });
        const el = document.querySelector("#copy-button");
        el.textContent = "Copied!";
        el.setAttribute("disabled", "true");
        el.classList.toggle(".btn-disabled");
    } else {
        alert('Your browser does not support the clipboard feature. Please copy the link manually.');
    }
}

async function reloadImg(url) {
    await fetch(url, {cache: 'reload', mode: 'same-origin'})
    document.body.querySelectorAll(`img[src='${url}']`)
        .forEach(img => img.src = url)

    if (navigator.userAgent.toLowerCase().includes('firefox')) {
        window.location.reload(true);
    }
}

function initGlobalKeyboardShortcuts() {
    document.addEventListener("keydown", (event) => {
        const key = event.key.toLowerCase();
        const viewRecipePage = window.location.pathname.match(/^\/recipes\/(\d+)$/);

        if (event.ctrlKey || event.metaKey) {
            if (event.altKey) {
                switch (key) {
                    case "d":
                        if (viewRecipePage) {
                            htmx.trigger("#duplicate-recipe", "click");
                        }
                        break;
                    case "i":
                        htmx.ajax("GET", "/recipes/add", {
                            target: "#content"
                        }).then(() => {
                            document.querySelector("#import-recipes-dialog").showModal();
                        }).then(() => {
                            window.history.pushState({}, "", "/recipes/add");
                        });
                        break;
                    case "n":
                        htmx.ajax("GET", "/recipes/add/manual", {
                            target: "#content"
                        }).then(() => {
                            window.history.pushState({}, "", "/recipes/add/manual");
                        });
                        break;
                    case "s":
                        if (window.location.pathname === "/recipes/add/manual" || window.location.pathname.match(/^\/recipes\/(\d+)\/edit$/)) {
                            const form = document.querySelector('form.card-body');
                            if (form) {
                                form.requestSubmit();
                            }
                        } else {
                            htmx.ajax("GET", "/settings", {
                                target: "#settings-dialog-content"
                            }).then(() => {
                                document.querySelector("#settings-dialog").showModal();
                            });
                        }
                        break;
                    case "r":
                        htmx.ajax("GET", "/reports", {
                            target: "#content"
                        }).then(() => {
                            window.history.pushState({}, "", "/reports");
                        });
                        break;
                    case "w":
                        htmx.ajax("GET", "/recipes/add", {
                            target: "#content"
                        }).then(() => {
                            document.querySelector("#websites-dialog").showModal();
                            window.history.pushState({}, "", "/recipes/add");
                        });
                        break;
                    default:
                        break;
                }
            } else if (event.shiftKey) {
                switch (key) {
                    case 'f':
                        if (viewRecipePage) {
                            const id = parseInt(viewRecipePage[1], 10);
                            htmx.trigger(`#favourite-${id}`, "click");
                        }
                        break;
                    default:
                        break;
                }
            } else {
                switch (key) {
                    case "e":
                        if (viewRecipePage) {
                            htmx.trigger("#edit-recipe", "click");
                        }
                        break;
                    case "s":
                        event.preventDefault();
                        event.stopPropagation();

                        if (window.location.pathname === "/recipes/add/manual" || window.location.pathname.match(/^\/recipes\/(\d+)\/edit$/)) {
                            const form = document.querySelector('form.card-body');
                            if (form) {
                                form.requestSubmit();
                            }
                        }
                        break;
                    case "x":
                        if (viewRecipePage) {
                            const url = `/recipes/${parseInt(viewRecipePage[1], 10)}/share`;
                            htmx.ajax("POST", url, {
                                target: "#content"
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
                    fetch(`/recipes/${parseInt(viewRecipePage[1], 10)}`, {method: "DELETE"}).then(() => {
                        window.location.replace("/recipes");
                    });
                }
            }
        }
    });
}

function initTheme(themeDefault, themeSelected) {
    let theme = themeSelected === "default" ? themeDefault : themeSelected;
    localStorage.setItem("theme", theme);
    document.documentElement.setAttribute("data-theme", theme);
}

function syncLayout() {
    const isAside = document.querySelector("#data-layout").attributes.getNamedItem("data-layout").value === "with-aside";

    ["desktop-nav", "mobile-nav", "add-recipe", "pagination"].forEach((id) => {
        document.getElementById(id)?.classList.toggle("hidden", !isAside);
    });
}

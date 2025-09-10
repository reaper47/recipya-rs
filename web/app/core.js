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

function initRecipeFormJS() {
    HtmlDurationPicker.init()

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
                forceFallback: true,
                chosenClass: 'is-chosen',
                dragClass: 'is-dragging',
                ghostClass: 'is-ghost',
                onStart(_event) {
                    document.body.classList.add('dragging');
                },
                onEnd(_event) {
                    document.body.classList.remove('dragging');
                },
            });
        }
    });
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

async function copyText(buttonId, sourceId) {
    const button = document.getElementById(buttonId);
    const source = document.getElementById(sourceId);

    if (!button || !source) return;

    try {
        await navigator.clipboard.writeText(source.value);
        button.textContent = 'Copied!';
    } catch (_) {
        source.select();
        document.execCommand('copy');
        button.textContent = 'Copied!';
    }

    setTimeout(() => {
        button.textContent = 'Copy';
    }, 1200);
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

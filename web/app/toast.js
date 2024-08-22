var timeoutToast = timeoutToast || null;

function receiveToastMessage(event) {
    const { title, message, background, action } = JSON.parse(event.detail.value);
    showToast(title, message, background, action);

    if (title === "Software updated") {
        setTimeout(() => window.location.reload(1), 6000);
    }
}

function showToast(title, message, background, action) {
    const toast = document.querySelector("#toast_alert").cloneNode(true);
    toast.classList.remove("alert-error", "alert-info", "alert-success", "alert-warning", background, "hidden");
    toast.classList.add(background, "toast-displayed");
    toast.id = "";

    toast.querySelector("#toast_title").innerHTML = title;
    toast.querySelector("#toast_message").innerText = message;
    switch (background) {
        case "alert-error":
            toast.querySelector("#toast_icon").innerHTML = `<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"></path>`;
            break;
        case "alert-info":
            toast.querySelector("#toast_icon").innerHTML = `<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>`;
            break;
        case "alert-warning":
            toast.querySelector("#toast_icon").innerHTML = `<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />`;
            break;
    }

    const dialogs = document.querySelectorAll('dialog[open]');
    let container = document.querySelector('#toast_container');
    if (dialogs.length > 0) {
        container = dialogs[0].querySelector('.toast-container-dialog');
    }

    const button = toast.querySelector("#toast_button");
    if (action) {
        const split = action.split(" ");
        if (split.length === 2) {
            button.setAttribute("hx-get", split[1]);
            button.setAttribute("hx-target", "#content");
            button.setAttribute("hx-push-url", split[1]);
            htmx.process(document.querySelector("#toast_button"));
        }
        button.innerText = split[0];
        button.classList.remove("hidden");
        toast.setAttribute("_", "on click remove me")
    } else {
        button.removeAttribute("hx-get");
        button.removeAttribute("hx-target");
        button.removeAttribute("hx-push-url");
        htmx.process(document.querySelector("#toast_button"));

        timeoutToast = setTimeout(() => {
            let opacity = 1;
            const id = setInterval(function () {
                if (opacity <= 0.1) {
                    clearInterval(id);
                    toast.parentNode.removeChild(toast);
                } else {
                    opacity -= 0.1;
                    toast.style.opacity = opacity;
                }
            }, 25)

            if (container.childElementCount <= 1) {
                container.classList.add('hidden');
            }
        }, 5000);
        button.classList.add("hidden");
    }

    htmx.process(toast);
    _hyperscript.processNode(toast);

    container.classList.remove('hidden');
    container.append(toast);
}

htmx.on('showToast', receiveToastMessage)
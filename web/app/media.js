if (!window.__mediaLoaded) {
  window.__mediaLoaded = true;

  function addMedia(event) {
    let labels = document.querySelectorAll("#media label");
    for (let i = 0; i < labels.length; i++) {
      const fileEl = labels[i].querySelector('input[type="file"]');
      const existingEl = labels[i].querySelector(
        'input[name="media-existing-image"]',
      );
      if (fileEl.files.length === 0 && !existingEl) {
        alert(`Please select an image or video for 'Media ${i + 1}'`);
        return;
      }
    }

    const buttons = document.querySelectorAll(".buttons-container button");
    buttons.forEach((btn) => btn.classList.remove("btn-active"));

    const cloneMedia = labels[labels.length - 1].cloneNode(true);
    labels.forEach((label) => label.classList.add("hidden"));
    cloneMedia.querySelector("input").value = "";
    cloneMedia.id = `media-${labels.length + 1}`;

    let img = cloneMedia.querySelector("img");
    if (!img) {
      img = cloneMedia.querySelector("video");
      img.removeAttribute("style");
    }
    img.src = "";

    cloneMedia.querySelector("div").classList.remove("hidden");
    cloneMedia.querySelector("input[type='file']").removeAttribute("disabled");
    cloneMedia.querySelector("input[type='url']").value = "";
    cloneMedia.querySelector("span > div").classList.remove("hidden");
    cloneMedia.querySelector(".image-actions").classList.add("hidden");
    cloneMedia.querySelector(".cropper-wrap")?.classList.add("hidden");
    cloneMedia.querySelector("input[name='media-existing-image']")?.remove();
    document.querySelector("#media").appendChild(cloneMedia);

    const video = cloneMedia.querySelector("video");
    if (video) {
      img.classList.remove("hidden");
      video.parentNode.removeChild(video);
    }
    labels = document.querySelectorAll("#media label");
    labels[labels.length - 1].classList.remove("hidden");
    _hyperscript.processNode(cloneMedia);

    let target = event.target;
    while (["svg", "circle"].includes(target.tagName)) {
      target = target.parentElement;
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
    const label = getLabel(event);
    const img = label.querySelector("img");
    img.src = "";
    img.value = "";

    label.querySelector(".cropper-wrap")?.removeAttribute("style");

    if (img.nextElementSibling && "VIDEO" === img.nextElementSibling.tagName) {
      img.parentElement.removeChild(img.nextElementSibling);
      img.classList.remove("hidden");
    }

    const span = label.querySelector("span");
    const fileInput = span.querySelector("input[type=file]");

    fileInput.value = null;
    fileInput.files = new DataTransfer().files;
    fileInput.disabled = false;

    span.children[0].classList.remove("hidden");
    span.querySelector(".image-actions")?.classList.add("hidden");
    label.querySelector("cropper-canvas")?.remove();
    label.querySelector("input[name='media-existing-image']")?.remove();
  }

  function switchMedia(
    event,
    mediaId = "#media",
    buttonsContainer = ".buttons-container",
  ) {
    let target = event.target;
    if (target.tagName === "SVG") {
      target = target.parentElement;
    }

    const label = document.querySelectorAll(`${mediaId} label`);
    const targetId = target.id.replace("button-", "");

    for (let i = 0; i < label.length; i++) {
      if (label[i].id === targetId) {
        label.forEach((e) => e.classList.add("hidden"));
        label[i].classList.remove("hidden");
        document
          .querySelectorAll(`${buttonsContainer} button`)
          .forEach((e) => e.classList.remove("btn-active"));
        document.getElementById(event.target.id).classList.add("btn-active");
        return;
      }
    }

    document.getElementById(event.target.id).classList.add("btn-active");
  }

  function updateMediaFromFetch(input, url) {
    if (url === window.location.href) {
      input.value = "";
      return;
    }

    const pathRegexImage =
      /^\/data\/images\/([\da-z]{8}-([\da-z]{4}-){3}[\da-z]{12}).webp$/;
    const pathRegexVideo =
      /^\/data\/videos\/([\da-z]{8}-([\da-z]{4}-){3}[\da-z]{12}).webm$/;
    const urlObject = new URL(url);

    let uuid;
    if (pathRegexImage.test(urlObject.pathname)) {
      uuid = urlObject.pathname.match(pathRegexImage)[1];
    } else if (pathRegexVideo.test(urlObject.pathname)) {
      uuid = urlObject.pathname.match(pathRegexVideo)[1];
    } else {
      uuid = crypto.randomUUID();
    }
    fetch(url)
      .then((response) => response.blob())
      .then((blob) => {
        const file = new File([blob], uuid, { type: blob.type });
        const dataTransfer = new DataTransfer();
        dataTransfer.items.add(file);
        input.files = dataTransfer.files;
        input.dispatchEvent(new Event("change"));
      });
  }

  class ImageEditor {
    constructor(labelEl) {
      this.container = labelEl;
      this.originalImage = labelEl.querySelector("img");

      return this._init();
    }

    apply() {
      const selection = this._selection();
      const devicePixelRatio = window.devicePixelRatio || 1;

      selection
        .$toCanvas({
          width: Math.round(selection.width * devicePixelRatio),
          height: Math.round(selection.height * devicePixelRatio),
          beforeDraw: (context, _canvas) => {
            context.imageSmoothingEnabled = true;
            context.imageSmoothingQuality = "high";
          },
        })
        .then((canvas) => {
          canvas.style.width = `${selection.width}px`;
          canvas.style.height = `${selection.height}px`;

          canvas.toBlob(
            (blob) => {
              const objectURL = URL.createObjectURL(blob);
              this.originalImage.src = objectURL;

              if (this.originalImage.dataset.objectUrl) {
                URL.revokeObjectURL(this.originalImage.dataset.objectUrl);
              }
              this.originalImage.dataset.objectUrl = objectURL;

              const file = new File([blob], "edited-image.png", {
                type: "image/png",
                lastModified: Date.now(),
              });

              const hiddenInput = this.container.querySelector(
                'input[name="media"]',
              );
              const dt = new DataTransfer();
              dt.items.add(file);
              hiddenInput.files = dt.files;
            },
            "image/png",
            1.0,
          );

          this.originalImage.parentElement.querySelector("input")?.remove();
          this.destroy();
        });
    }

    destroy() {
      this._canvas().remove();
      this._toggleToolboxActions(false);
      this.originalImage.style.display = "block";
      setTimeout(() => {
        this._setWrapperDimensions(this.originalImage);
      }, 50);
    }

    setHandle(mode) {
      this._canvas()
        .querySelector("cropper-handle")
        .setAttribute("action", mode);
    }

    rotate(factor) {
      this._img().$rotate(factor);
    }

    scale(x, y) {
      this._img().$scale(x, y);
    }

    zoom(factor) {
      this._img().$zoom(factor);
    }

    _init() {
      const img = this.container.querySelector("img");
      this._setWrapperDimensions(img);
      this._toggleToolboxActions(true);

      this.cropper = new window.Cropper.default(img, {
        template: `
      <cropper-canvas background>
        <cropper-image contain scalable rotatable translatable></cropper-image>
        <cropper-shade hidden></cropper-shade>
        <cropper-handle action="select" plain></cropper-handle>
        <cropper-selection movable resizable initial-coverage="0.5" outlined>
          <cropper-grid role="grid" bordered covered></cropper-grid>
          <cropper-crosshair centered></cropper-crosshair>
          <cropper-handle action="move" theme-color="rgba(255,255,255,0.35)"></cropper-handle>
          <cropper-handle action="n-resize"></cropper-handle>
          <cropper-handle action="e-resize"></cropper-handle>
          <cropper-handle action="s-resize"></cropper-handle>
          <cropper-handle action="w-resize"></cropper-handle>
          <cropper-handle action="ne-resize"></cropper-handle>
          <cropper-handle action="nw-resize"></cropper-handle>
          <cropper-handle action="se-resize"></cropper-handle>
          <cropper-handle action="sw-resize"></cropper-handle>
        </cropper-selection>
      </cropper-canvas>`,
      });

      return this;
    }

    _canvas() {
      return this.cropper.getCropperCanvas();
    }

    _img() {
      return this.cropper.getCropperImage();
    }

    _selection() {
      return this.cropper.getCropperSelection();
    }

    _setWrapperDimensions(img) {
      const { width: originalWidth, height: originalHeight } =
        img.getBoundingClientRect();
      const wrapper = this.container.querySelector(".cropper-wrap");
      wrapper.style.height = originalHeight + "px";
      wrapper.style.width = originalWidth + "px";
    }

    _toggleToolboxActions(isEnabled) {
      const buttons = Array.from(
        this.container.querySelector(".image-actions .main-toolbox").children,
      );
      const editToolbox = this.container.querySelector(".edit-toolbox");

      if (isEnabled) {
        buttons.forEach((e) => e.classList.add("hidden"));
        editToolbox.classList.remove("hidden");
      } else {
        buttons.forEach((btn) => btn.classList.remove("hidden"));
        editToolbox.classList.add("hidden");
      }
    }
  }

  function editImage(event) {
    window.imageEditor = new ImageEditor(getLabel(event));
    toggleMediaButtons(event, false);
  }

  function toggleMediaButtons(event, isEnabled) {
    const mediaButtons = document.querySelectorAll("[id^='media-button-']");
    const addMediaButton = document.querySelector("#add-media-button");
    const input = getLabel(event).querySelector('input[name="media"]');

    if (isEnabled) {
      mediaButtons.forEach((button) => {
        button.removeAttribute("disabled");
      });
      addMediaButton.removeAttribute("disabled");
      input.removeAttribute("disabled");
    } else {
      mediaButtons.forEach((button) => {
        button.setAttribute("disabled", "true");
      });
      addMediaButton.setAttribute("disabled", "true");
      input.setAttribute("disabled", "true");
    }
  }

  function getLabel(event) {
    return event.target.closest("label");
  }

  function cancelCropper(event) {
    window.imageEditor.destroy();
    toggleMediaButtons(event, true);
  }

  function applyCropper(event) {
    window.imageEditor.apply();
    window.imageEditor = null;
    toggleMediaButtons(event, true);
  }
}

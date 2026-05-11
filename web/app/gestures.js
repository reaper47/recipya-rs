function maybeInitRecipe() {
  if (/^\/recipes\/\d+$/.test(window.location.pathname)) {
    initRecipeViewJS();
  }
}

document.addEventListener("htmx:afterSettle", maybeInitRecipe);

function initDrag(el, onReorder) {
  if (!el) {
    return;
  }

  let dragSrc = null;
  let ghost = null;
  let touchRow = null;
  let rowOffsets = [];
  let rafId = null;

  el.addEventListener("dragstart", (event) => {
    const row = event.target.closest("[data-drag-row]");
    if (!row) {
      return;
    }

    dragSrc = row;
    row.querySelector("[data-drageable]").classList.add("dragging");
    event.dataTransfer.effectAllowed = "move";
  });

  el.addEventListener("dragover", (event) => {
    event.preventDefault();

    const target = event.target.closest("[data-drag-row]");
    if (!target || target === dragSrc) {
      return;
    }

    clearDragOver(el);
    target.querySelector("[data-drageable]").classList.add("drag-over");
    insertAt(el, dragSrc, target, event.clientY);
  });

  el.addEventListener("dragend", () => {
    dragSrc?.querySelector("[data-drageable]").classList.remove("dragging");
    clearDragOver(el);
    dragSrc = null;
    onReorder?.(getOrder(el));
  });

  el.addEventListener(
    "touchstart",
    (event) => {
      if (!event.target.closest("[data-drag-handle]")) {
        return;
      }

      event.preventDefault();
      touchRow = event.target.closest("[data-drag-row]");
      touchRow.querySelector("[data-drageable]").classList.add("dragging");

      const rect = touchRow.getBoundingClientRect();
      offX = event.touches[0].clientX - rect.left;
      offY = event.touches[0].clientY - rect.top;

      rowOffsets = [...el.querySelectorAll("[data-drag-row]")].map((el) => ({
        el,
        top: el.getBoundingClientRect().top,
        bottom: el.getBoundingClientRect().bottom,
      }));

      ghost = createGhost(touchRow, rect, el);
      el.appendChild(ghost);
    },
    { passive: false },
  );

  document.addEventListener(
    "touchmove",
    (event) => {
      if (!touchRow) {
        return;
      }

      event.preventDefault();

      const touch = event.touches[0];
      requestAnimationFrame(() => {
        const dx = moveEvent.clientX - startX;
        cancelAnimationFrame(rafId);
        rafId = requestAnimationFrame(() => {
          item.style.transform = `translate3d(${dx}px, 0, 0)`;
        });
      });

      const target = rowOffsets.find(
        (row) => touch.clientY >= row.top && touch.clientY <= row.bottom,
      )?.el;

      if (target && target !== touchRow) {
        clearDragOver(el);
        target.querySelector("[data-drageable]").classList.add("drag-over");
        insertAt(el, touchRow, target, touch.clientY);

        rowOffsets = [...el.querySelectorAll("[data-drag-row]")].map((el) => ({
          el,
          top: el.getBoundingClientRect().top,
          bottom: el.getBoundingClientRect().bottom,
        }));
      }
    },
    { passive: false },
  );

  document.addEventListener("touchend", () => {
    if (!touchRow) {
      return;
    }

    touchRow.querySelector("[data-drageable]").classList.remove("dragging");
    clearDragOver(el);
    ghost?.remove();
    ghost = null;
    touchRow = null;
    onReorder?.(getOrder(el));
  });
}

function insertAt(list, dragged, target, clientY) {
  if (!dragged) {
    return;
  }

  const mid = target.getBoundingClientRect().top + target.offsetHeight / 2;
  list.insertBefore(dragged, clientY < mid ? target : target.nextSibling);
}

function clearDragOver(list) {
  list
    .querySelectorAll(".drag-over")
    .forEach((el) => el.classList.remove("drag-over"));
}

function getOrder(list) {
  return [...list.querySelectorAll("[data-drag-row]")].map((r) => r.dataset.id);
}

function initSwipe(el, { onDelete, onComplete }) {
  if (!el) {
    return;
  }

  const DISMISS_THRESHOLD = 100;

  el.addEventListener("dragstart", (event) => event.preventDefault());

  el.addEventListener("pointerdown", (event) => {
    if (
      (event.pointerType === "mouse" && event.button !== 0) ||
      event.target.closest("[data-drag-handle]") ||
      event.target.tagName === "INPUT" ||
      event.target.tagName === "BUTTON"
    ) {
      return;
    }

    const item = event.target.closest("[data-swipeable]");
    if (!item) {
      return;
    }

    const captureTarget = event.target;
    const pid = event.pointerId;

    try {
      captureTarget.setPointerCapture(pid);
    } catch (err) {
      return;
    }

    let startX = event.clientX;
    let currentX = 0;
    let isDecided = false;
    let isSwiping = false;
    let captured = false;

    const onMove = (moveEvent) => {
      const dx = moveEvent.clientX - startX;

      if (!isDecided) {
        if (Math.abs(dx) > 8) {
          isDecided = true;
          isSwiping = true;
          try {
            captureTarget.setPointerCapture(pid);
            captured = true;
          } catch (err) {}
          document.body.classList.add("is-swiping");
        }
      }

      if (!isSwiping) {
        return;
      }

      currentX = dx;
      item.style.transform = `translate3d(${dx}px, 0, 0)`;

      const progress = Math.min(Math.abs(dx) / DISMISS_THRESHOLD, 1);
      const row = item.closest(".list-row");
      row
        ?.querySelector(dx < 0 ? ".bg-delete" : ".bg-complete")
        ?.style.setProperty("opacity", progress);
    };

    const onEnd = () => {
      if (captured && captureTarget.hasPointerCapture(pid)) {
        try {
          captureTarget.releasePointerCapture(pid);
        } catch (err) {}
      }

      document.body.classList.remove("is-swiping");

      const row = item.closest(".list-row");
      if (currentX < -DISMISS_THRESHOLD) {
        onDelete?.(row);
      } else if (currentX > DISMISS_THRESHOLD) {
        onComplete?.(row);
      }

      item.style.transition = "transform 0.3s cubic-bezier(0.2, 1, 0.3, 1)";
      item.style.transform = "translate3d(0, 0, 0)";

      row
        ?.querySelectorAll(".bg-delete, .bg-complete")
        .forEach((bg) => (bg.style.opacity = "0"));

      item.addEventListener(
        "transitionend",
        () => (item.style.transition = ""),
        { once: true },
      );

      document.removeEventListener("pointermove", onMove);
      document.removeEventListener("pointerup", onEnd);
      document.removeEventListener("pointercancel", onEnd);
    };

    document.addEventListener("pointermove", onMove);
    document.addEventListener("pointerup", onEnd);
    document.addEventListener("pointercancel", onEnd);
  });
}

function dismissRow(row, dir) {
  if (!row) {
    return;
  }

  Object.assign(row.style, {
    transition: "all 0.4s cubic-bezier(0.4, 0, 0.2, 1)",
    transform: `translate3d(${dir > 0 ? 100 : -100}%, 0, 0) scaleY(0)`,
    opacity: 0,
    pointerEvents: "none",
  });

  setTimeout(() => row.remove(), 400);
}

function createGhost(row, rect, list) {
  const div = document.createElement("div");
  const clientRect = list.getBoundingClientRect();

  div.className = "drag-ghost";
  div.textContent = row.querySelector(".item-label")?.textContent || "";

  Object.assign(div.style, {
    position: "absolute",
    pointerEvents: "none",
    zIndex: 999,
    width: `${rect.width}px`,
    height: `${rect.height}px`,
    left: `0px`,
    top: `0px`,
    transform: `translate3d(${rect.left - clientRect.left}px, ${rect.top - clientRect.top}px, 0)`,
    opacity: 0.9,
    background: "white",
    border: "1px solid #ccc",
    display: "flex",
    alignItems: "center",
    padding: "0 10px",
  });

  return div;
}

function insertAt(list, dragged, target, clientY) {
  if (dragged) {
    const mid = target.getBoundingClientRect().top + target.offsetHeight / 2;
    list.insertBefore(dragged, clientY < mid ? target : target.nextSibling);
  }
}

function clearDragOver(list) {
  list
    .querySelectorAll(".drag-over")
    .forEach((el) => el.classList.remove("drag-over"));
}

function getOrder(list) {
  return [...list.querySelectorAll("[data-drag-row]")].map((r) => r.dataset.id);
}

var wakeLock = null;

let isToggledByUser = false;
let isVisibilityStateHidden = false;
let isWakeLockActivated = false;

function toggleWakeLock(baseIdIcon = "bulb") {
  isToggledByUser = true;
  if (wakeLock === null) {
    requestWakeLock(true, baseIdIcon);
  } else {
    wakeLock.release().then(() => {
      isToggledByUser = false;
    });
  }
}

function requestWakeLock(displayToast, baseIdIcon = "bulb") {
  navigator.wakeLock
    ?.request("screen")
    .then((lock) => {
      wakeLock = lock;

      if (isToggledByUser) {
        isWakeLockActivated = true;
      }

      if (displayToast) {
        showToast("", "Screen lock activated.", "alert-info");
      }

      toggleWakeLockIcon(true, baseIdIcon);
      isToggledByUser = false;

      wakeLock.addEventListener("release", () => {
        wakeLock = null;

        if (isToggledByUser) {
          isWakeLockActivated = false;
        }

        if (!isVisibilityStateHidden) {
          showToast("", "Screen lock deactivated.", "alert-warning");
        }

        toggleWakeLockIcon(false, baseIdIcon);
        displayToast = false;
      });
    })
    .catch((err) => {
      showToast("", "Failed to toggle screen lock.", "alert-error");
      console.error(`Screen lock error: ${err.name}, ${err.message}`);
    });
}

function toggleWakeLockIcon(isActivated, baseIdIcon = "bulb") {
  const onEl = document.getElementById(`${baseIdIcon}-on`);
  const offEl = document.getElementById(`${baseIdIcon}-off`);

  if (isActivated) {
    onEl.classList.remove("hidden");
    offEl.classList.add("hidden");
  } else {
    onEl.classList.add("hidden");
    offEl.classList.remove("hidden");
  }
}

document.addEventListener("visibilitychange", () => {
  if (document.visibilityState === "hidden") {
    isVisibilityStateHidden = true;
  } else if (document.visibilityState === "visible") {
    if (isVisibilityStateHidden) {
      isVisibilityStateHidden = false;
    }

    if (isWakeLockActivated) {
      requestWakeLock(false);
    }
  }
});

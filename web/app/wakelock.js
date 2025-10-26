var wakeLock = null;
let isWakeLockEnabled = false;

initWakeLock(false);

function initWakeLock(displayToast = true) {
    isWakeLockEnabled = true;
    requestWakeLock(displayToast);
}

function requestWakeLock(displayToast = true) {
    if (!isWakeLockEnabled) {
        return;
    }

    navigator.wakeLock?.request("screen")
        .then((lock) => {
            wakeLock = lock;

            wakeLock.addEventListener('release', () => {
                wakeLock = null;

                if (isWakeLockEnabled) {
                    showToast("", "Screen lock deactivated.", "alert-warning");
                }
            });

            if (displayToast) {
                showToast("", "Screen lock activated.", "alert-info");
            }
        })
        .catch((err) => {
            showToast("", "Failed to toggle screen lock.", "alert-error");
            console.log(`Screen lock error: ${err.name}, ${err.message}`);
        });
}

document.addEventListener('visibilitychange', () => {
    if (document.visibilityState === 'visible' && isWakeLockEnabled && !wakeLock) {
        requestWakeLock(false);
    }
});

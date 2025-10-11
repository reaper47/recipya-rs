var wakeLock = null;
initWakeLock(false);

function initWakeLock(displayToast = true) {
    navigator.wakeLock?.request("screen")
        .then((lock) => {
            wakeLock = lock;
            wakeLock.onrelease = () => {
                wakeLock = null;
                showToast("", "Screen lock deactivated.", "alert-warning");
            };

            if (displayToast) {
                showToast("", "Screen lock activated.", "alert-info");
            }
        })
        .catch((err) => {
            showToast("", "Failed to toggle screen lock.", "alert-error");
            console.log(`Screen lock error: ${err.name}, ${err.message}`);
        });
}

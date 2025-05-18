var wakeLock = null;
initWakeLock();

function initWakeLock() {
    navigator.wakeLock?.request("screen")
        .then((lock) => {
            wakeLock = lock;
            wakeLock.onrelease = () => {
                wakeLock = null;
                console.info("Screen lock deactivated.");
            };
            console.info("Screen lock activated.");
        })
        .catch((err) => {
            console.log(`Screen lock error: ${err.name}, ${err.message}`);
        });
}

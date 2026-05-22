if (!window.__timerLoaded) {
  window.__timerLoaded = true;

  let timers = {};

  function initTimer(event) {
    const container = getTimerContainer(event);
    timers[container.id] = new Timer(container);
  }

  function playTimer(event) {
    const { id } = getTimerContainer(event);
    timers[id].play();
  }

  function pauseTimer(event) {
    const { id } = getTimerContainer(event);
    timers[id].pause();
  }

  function stopTimer(event) {
    const { id } = getTimerContainer(event);
    timers[id].stop();
    setTimeout(() => {
      timers[id].destroy();
      timers[id] = null;
    }, 200);
  }

  function getTimerContainer(event) {
    let el = event.target;
    while (!el.classList.contains("timer-container")) {
      el = el.parentElement;
    }
    return el;
  }

  class Timer {
    /**
     * @param {string|HTMLElement} mainContainer - element id or the element itself
     */
    constructor(mainContainer) {
      const root =
        typeof mainContainer === "string"
          ? document.getElementById(mainContainer)
          : mainContainer;

      if (!root) {
        throw new Error(`Timer container "${mainContainer}" not found`);
      }

      this.mainContainer = root.lastElementChild;
      this.countdownContainer = this.mainContainer.firstElementChild;
      this.buttonsContainer = this.countdownContainer.nextSibling;

      const [hoursEl, minutesEl, secondsEl] =
        this.countdownContainer.querySelectorAll("span");
      this.hoursEl = hoursEl;
      this.minutesEl = minutesEl;
      this.secondsEl = secondsEl;

      this.hours = parseInt(this.hoursEl.textContent, 10) || 0;
      this.minutes = parseInt(this.minutesEl.textContent, 10) || 0;
      this.seconds = parseInt(this.secondsEl.textContent, 10) || 0;

      this.originalHours = this.hours;
      this.originalMinutes = this.minutes;
      this.originalSeconds = this.seconds;

      this.isTimerInitialized = false;
      this.interval = null;
      this.isRunning = false;
      this.isMelodyPlaying = false;

      this.audioContext = new (
        window.AudioContext || window.webkitAudioContext
      )();
      this.activeOscillators = [];
      this.activeGainNodes = [];
      this.alarmSound = this._createAlarmSound();

      this._init();
    }

    play() {
      this.interval = setInterval(() => this._tick(), 1000);
      this.isRunning = true;
      if (this.isTimerInitialized) {
        this.alarmSound.playStartBeep();
      }
    }

    pause() {
      this.alarmSound.playPauseBeep();
      clearInterval(this.interval);
      this.interval = null;
      this.isRunning = false;
    }

    stop() {
      this.alarmSound.playStopBeep();
      clearInterval(this.interval);
      this.interval = null;
      this.isRunning = false;
      this._updateDisplay(true);
    }

    destroy() {
      if (this.interval) {
        clearInterval(this.interval);
      }

      this._stopSound();

      if (this.audioContext) {
        this.audioContext.close();
      }
    }

    _init() {
      this._toggleState();
      this.alarmSound.playInitBeep();
      this.isTimerInitialized = true;
    }

    _createAlarmSound() {
      return {
        playInitBeep: () => {
          this._stopSound();
          const audioContext = this.audioContext;
          const oscillator = audioContext.createOscillator();
          const gainNode = audioContext.createGain();

          oscillator.connect(gainNode);
          gainNode.connect(audioContext.destination);

          oscillator.frequency.value = 800;
          oscillator.type = "square";

          gainNode.gain.setValueAtTime(0.3, audioContext.currentTime);
          gainNode.gain.exponentialRampToValueAtTime(
            0.01,
            audioContext.currentTime + 0.5,
          );

          oscillator.start(audioContext.currentTime);
          oscillator.stop(audioContext.currentTime + 0.5);
        },
        playMelody: () => {
          this.isMelodyPlaying = true;
          this._stopSound();
          const audioContext = this.audioContext;

          const c5 = 523.25;
          const a5sharp = 466.1638;
          const c6 = 1046.5;
          const c6sharp = 554.3653;
          const d6sharp = 622.254;

          const sixteenthNote = 0.175;
          const quarterNote = sixteenthNote * 4;

          const melody = [
            { note: c5, duration: sixteenthNote },
            { note: c5, duration: sixteenthNote },
            { note: c5, duration: sixteenthNote },
            { note: c5, duration: sixteenthNote },
            { note: c6, duration: quarterNote },

            { note: c5, duration: sixteenthNote },
            { note: c5, duration: sixteenthNote },
            { note: c5, duration: sixteenthNote },
            { note: c5, duration: sixteenthNote },
            { note: c6, duration: quarterNote },

            { note: c5, duration: sixteenthNote },
            { note: c5, duration: sixteenthNote },
            { note: c5, duration: sixteenthNote },
            { note: c5, duration: sixteenthNote },
            { note: c6, duration: sixteenthNote },

            { note: c5, duration: sixteenthNote },
            { note: c6sharp, duration: sixteenthNote },
            { note: c5, duration: sixteenthNote },
            { note: c6, duration: sixteenthNote },
            { note: c5, duration: sixteenthNote },
            { note: a5sharp, duration: sixteenthNote },
            { note: c5, duration: sixteenthNote },
            { note: c6, duration: sixteenthNote },
            { note: c5, duration: sixteenthNote },
            { note: c6sharp, duration: sixteenthNote },
            { note: c5, duration: sixteenthNote },
            { note: d6sharp, duration: quarterNote },
          ];

          let currentTime = audioContext.currentTime;
          let totalDuration = 0;

          melody.forEach((tone) => {
            const oscillator = this.audioContext.createOscillator();
            const gainNode = this.audioContext.createGain();

            oscillator.connect(gainNode);
            gainNode.connect(this.audioContext.destination);

            oscillator.frequency.value = tone.note;
            oscillator.type = "sine";

            gainNode.gain.setValueAtTime(0, currentTime);
            gainNode.gain.linearRampToValueAtTime(0.3, currentTime + 0.02);
            gainNode.gain.exponentialRampToValueAtTime(
              0.01,
              currentTime + tone.duration,
            );

            oscillator.start(currentTime);
            oscillator.stop(currentTime + tone.duration);

            this.activeOscillators.push(oscillator);
            this.activeGainNodes.push(gainNode);

            oscillator.onended = () => {
              const oscIndex = this.activeOscillators.indexOf(oscillator);
              const gainIndex = this.activeGainNodes.indexOf(gainNode);

              if (oscIndex > -1) {
                this.activeOscillators.splice(oscIndex, 1);
              }
              if (gainIndex > -1) {
                this.activeGainNodes.splice(gainIndex, 1);
              }
            };

            currentTime += tone.duration;
            totalDuration += tone.duration;
          });

          setTimeout(() => {
            this.isMelodyPlaying = false;
          }, totalDuration * 1000);
        },
        playStartBeep: () => {
          this._stopSound();
          this._createSingleBeep(2000, "sawtooth");
        },
        playPauseBeep: () => {
          this._stopSound();
          this._createSingleBeep(1800, "sawtooth");
        },
        playStopBeep: () => {
          this._stopSound();
          this._createSingleBeep(200, "sine");
        },
      };
    }

    _stopSound() {
      this.activeOscillators.forEach((osc) => {
        try {
          osc?.stop();
          osc?.disconnect();
        } catch (_) {}
      });

      const now = this.audioContext.currentTime;
      this.activeGainNodes.forEach((gain) => {
        try {
          gain.gain.cancelScheduledValues(now);
          gain.gain.setValueAtTime(gain.gain.value, now);
          gain.gain.linearRampToValueAtTime(0, now + 0.01);
          gain.disconnect();
        } catch (_) {}
      });

      this.activeOscillators = [];
      this.activeGainNodes = [];
    }

    _createSingleBeep(frequency, oscillatorType) {
      const audioContext = this.audioContext;
      const oscillator = audioContext.createOscillator();
      const gainNode = audioContext.createGain();

      oscillator.connect(gainNode);
      gainNode.connect(audioContext.destination);

      oscillator.frequency.value = frequency;
      oscillator.type = oscillatorType;

      gainNode.gain.setValueAtTime(0.3, audioContext.currentTime);
      gainNode.gain.exponentialRampToValueAtTime(
        0.01,
        audioContext.currentTime + 0.5,
      );

      oscillator.start(audioContext.currentTime);
      oscillator.stop(audioContext.currentTime + 0.5);
    }

    _tick() {
      if (this.hours === 0 && this.minutes === 0 && this.seconds === 0) {
        clearInterval(this.interval);
        this.interval = setInterval(() => {
          if (!this.isMelodyPlaying) {
            this.alarmSound.playMelody();
          }
        }, 2000);
        this.alarmSound.playMelody();
        this.interval = null;
        this.isRunning = false;
        this.buttonsContainer
          .querySelector(".timer-pause")
          .classList.add("hidden");
        this.buttonsContainer
          .querySelector(".timer-stop")
          .classList.add("hidden");
        this.buttonsContainer
          .querySelector(".timer-end")
          .classList.remove("hidden");
        return;
      }

      this.seconds--;
      if (this.seconds < 0) {
        this.seconds = 59;
        this.minutes--;
        if (this.minutes < 0) {
          this.minutes = 59;
          this.hours--;
        }
      }

      this._updateDisplay();
    }

    _updateDisplay(isResetToOriginal = false) {
      let hours = this.hours;
      let minutes = this.minutes;
      let seconds = this.seconds;

      if (isResetToOriginal) {
        hours = this.originalHours;
        minutes = this.originalMinutes;
        seconds = this.originalSeconds;
      }

      this.hoursEl.style.setProperty("--value", hours);
      this.minutesEl.style.setProperty("--value", minutes);
      this.secondsEl.style.setProperty("--value", seconds);

      this.hoursEl.textContent = hours;
      this.minutesEl.textContent = minutes.toString().padStart(2, "0");
      this.secondsEl.textContent = seconds.toString().padStart(2, "0");
    }

    _toggleState() {
      if (!this.isRunning) {
        this.play();
      } else {
        this.stop();
      }
    }
  }
}

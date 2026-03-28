const { invoke } = window.__TAURI__.core;

// State
let currentMode = "password";
let separator = "-";
let history = [];

// DOM Elements
const passwordText = document.getElementById("password-text");
const copyBtn = document.getElementById("copy-btn");
const copyToast = document.getElementById("copy-toast");
const strengthBar = document.getElementById("strength-bar");
const strengthLabel = document.getElementById("strength-label");
const generateBtn = document.getElementById("generate-btn");
const lengthSlider = document.getElementById("length-slider");
const lengthValue = document.getElementById("length-value");
const countSlider = document.getElementById("count-slider");
const countValue = document.getElementById("count-value");
const wordCountSlider = document.getElementById("word-count-slider");
const wordCountValue = document.getElementById("word-count-value");
const historyList = document.getElementById("history-list");
const historyToggle = document.getElementById("history-toggle");
const historyArrow = document.getElementById("history-arrow");
const clearHistoryBtn = document.getElementById("clear-history");
const passwordOptions = document.getElementById("password-options");
const passphraseOptions = document.getElementById("passphrase-options");
const customSymbolsInput = document.getElementById("custom-symbols");
const customSymbolsRow = document.getElementById("custom-symbols-row");
const themeToggle = document.getElementById("theme-toggle");
const iconMoon = document.getElementById("icon-moon");
const iconSun = document.getElementById("icon-sun");

// ===== Theme =====
function setTheme(theme) {
  document.documentElement.setAttribute("data-theme", theme);
  localStorage.setItem("passgene-theme", theme);
  if (theme === "dark") {
    iconMoon.classList.add("hidden");
    iconSun.classList.remove("hidden");
  } else {
    iconSun.classList.add("hidden");
    iconMoon.classList.remove("hidden");
  }
}

themeToggle.addEventListener("click", () => {
  const current = document.documentElement.getAttribute("data-theme");
  setTheme(current === "dark" ? "light" : "dark");
});

// Load saved theme
const savedTheme = localStorage.getItem("passgene-theme") || "dark";
setTheme(savedTheme);

// ===== Mode Toggle =====
document.getElementById("mode-password").addEventListener("click", () => setMode("password"));
document.getElementById("mode-passphrase").addEventListener("click", () => setMode("passphrase"));

function setMode(mode) {
  currentMode = mode;
  document.querySelectorAll(".mode-btn").forEach(btn => btn.classList.remove("active"));
  document.getElementById(`mode-${mode}`).classList.add("active");
  passwordOptions.classList.toggle("hidden", mode !== "password");
  passphraseOptions.classList.toggle("hidden", mode !== "passphrase");
  generate();
}

// ===== Sliders =====
lengthSlider.addEventListener("input", () => {
  lengthValue.textContent = lengthSlider.value;
});
lengthSlider.addEventListener("change", generate);

countSlider.addEventListener("input", () => {
  countValue.textContent = countSlider.value;
});
countSlider.addEventListener("change", generate);

wordCountSlider.addEventListener("input", () => {
  wordCountValue.textContent = wordCountSlider.value;
});
wordCountSlider.addEventListener("change", generate);

// ===== Checkboxes =====
["opt-uppercase", "opt-lowercase", "opt-numbers", "opt-symbols", "opt-capitalize", "opt-include-number"].forEach(id => {
  document.getElementById(id).addEventListener("change", generate);
});

document.getElementById("opt-symbols").addEventListener("change", () => {
  customSymbolsRow.style.display = document.getElementById("opt-symbols").checked ? "block" : "none";
});

// ===== Separator Buttons =====
document.querySelectorAll(".sep-btn").forEach(btn => {
  btn.addEventListener("click", () => {
    document.querySelectorAll(".sep-btn").forEach(b => b.classList.remove("active"));
    btn.classList.add("active");
    separator = btn.dataset.sep;
    generate();
  });
});

// ===== Generate =====
generateBtn.addEventListener("click", generate);

async function generate() {
  try {
    let result;
    if (currentMode === "password") {
      const customSym = customSymbolsInput.value.trim();
      result = await invoke("generate_passwords", {
        options: {
          length: parseInt(lengthSlider.value),
          count: parseInt(countSlider.value),
          uppercase: document.getElementById("opt-uppercase").checked,
          lowercase: document.getElementById("opt-lowercase").checked,
          numbers: document.getElementById("opt-numbers").checked,
          symbols: document.getElementById("opt-symbols").checked,
          custom_symbols: customSym.length > 0 ? customSym : null,
        }
      });
    } else {
      result = await invoke("generate_passphrase", {
        options: {
          word_count: parseInt(wordCountSlider.value),
          separator: separator,
          capitalize: document.getElementById("opt-capitalize").checked,
          include_number: document.getElementById("opt-include-number").checked,
        }
      });
    }

    passwordText.textContent = result.passwords.join("\n");
    updateStrength(result.strength);
    addToHistory(result.passwords);
  } catch (err) {
    console.error("Generation error:", err);
  }
}

// ===== Strength =====
function updateStrength(strength) {
  const colors = {
    1: "var(--red)",
    2: "var(--orange)",
    3: "var(--yellow)",
    4: "var(--green)",
    5: "#16a34a",
  };
  const widths = { 1: "20%", 2: "40%", 3: "60%", 4: "80%", 5: "100%" };

  strengthBar.style.width = widths[strength.score] || "0%";
  strengthBar.style.background = colors[strength.score] || "var(--red)";
  strengthLabel.textContent = strength.level;
  strengthLabel.style.color = colors[strength.score] || "var(--text-secondary)";
}

// ===== History =====
function addToHistory(passwords) {
  for (const pw of passwords) {
    history.unshift(pw);
  }
  history = history.slice(0, 20);
  renderHistory();
}

function renderHistory() {
  historyList.innerHTML = "";
  // Show/hide clear button
  if (history.length > 0) {
    clearHistoryBtn.classList.remove("hidden");
  } else {
    clearHistoryBtn.classList.add("hidden");
  }
  for (const pw of history) {
    const item = document.createElement("div");
    item.className = "history-item";

    const text = document.createElement("span");
    text.className = "hist-password";
    text.textContent = pw;

    const btn = document.createElement("button");
    btn.className = "hist-copy";
    btn.textContent = "copy";
    btn.addEventListener("click", () => copyToClipboard(pw));

    item.appendChild(text);
    item.appendChild(btn);
    historyList.appendChild(item);
  }
}

// Clear history
clearHistoryBtn.addEventListener("click", () => {
  history = [];
  renderHistory();
});

// ===== Copy =====
copyBtn.addEventListener("click", () => {
  const text = passwordText.textContent;
  if (text && text !== "Click Generate to start") {
    copyToClipboard(text);
  }
});

async function copyToClipboard(text) {
  try {
    await navigator.clipboard.writeText(text);
    copyToast.classList.add("show");
    setTimeout(() => copyToast.classList.remove("show"), 1200);
  } catch {
    const textarea = document.createElement("textarea");
    textarea.value = text;
    document.body.appendChild(textarea);
    textarea.select();
    document.execCommand("copy");
    document.body.removeChild(textarea);
    copyToast.classList.add("show");
    setTimeout(() => copyToast.classList.remove("show"), 1200);
  }
}

// ===== History Toggle =====
historyToggle.addEventListener("click", () => {
  historyList.classList.toggle("hidden");
  historyArrow.classList.toggle("open");
});

// ===== Keyboard Shortcut =====
document.addEventListener("keydown", (e) => {
  if (e.key === "Enter" || e.key === " ") {
    if (document.activeElement === document.body) {
      e.preventDefault();
      generate();
    }
  }
});

// Generate on load
generate();

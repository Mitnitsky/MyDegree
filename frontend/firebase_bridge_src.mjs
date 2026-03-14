// Firebase v10 modular bridge — tree-shakeable, bundled by esbuild.
// Only imports the functions we actually use.
import { initializeApp } from "firebase/app";
import {
  getAuth,
  onAuthStateChanged,
  signOut,
  signInWithPopup,
  signInWithEmailAndPassword,
  createUserWithEmailAndPassword,
  GoogleAuthProvider,
} from "firebase/auth";
import {
  getFirestore,
  doc,
  getDoc,
  setDoc,
} from "firebase/firestore";

// firebase_config.js (loaded before this script) sets window.FIREBASE_CONFIG
const app = initializeApp(window.FIREBASE_CONFIG);
const auth = getAuth(app);
const db = getFirestore(app);
const googleProvider = new GoogleAuthProvider();

// ── Auth ──────────────────────────────────────────────

window.onAuthChange = function (cb) {
  onAuthStateChanged(auth, function (user) {
    if (user) {
      cb(
        JSON.stringify({
          uid: user.uid,
          displayName: user.displayName || "",
          email: user.email || "",
        })
      );
    } else {
      cb(null);
    }
  });
};

window.signOutUser = function () {
  return signOut(auth);
};

// ── Auth Widget (replaces FirebaseUI) ─────────────────

window.startAuthUI = function (elementId) {
  var container = document.getElementById(elementId);
  if (!container) return;

  container.innerHTML = "";

  // Styles
  var style = document.createElement("style");
  style.textContent =
    ".fui-container{font-family:Alef,Roboto,sans-serif;direction:rtl;padding:16px 0}" +
    ".fui-btn{display:flex;align-items:center;justify-content:center;gap:8px;width:100%;padding:10px 16px;border-radius:4px;font-size:15px;cursor:pointer;border:1px solid #dadce0;background:#fff;color:#3c4043;transition:background .2s}" +
    ".fui-btn:hover{background:#f7f8f8}" +
    ".fui-btn:disabled{opacity:.6;cursor:default}" +
    ".fui-btn-google img{width:18px;height:18px}" +
    ".fui-divider{display:flex;align-items:center;margin:16px 0}" +
    ".fui-divider hr{flex:1;border:none;border-top:1px solid #dadce0}" +
    ".fui-divider span{margin:0 12px;color:#5f6368;font-size:13px}" +
    ".fui-input{width:100%;padding:10px 12px;border:1px solid #dadce0;border-radius:4px;font-size:14px;direction:ltr;text-align:right;box-sizing:border-box;margin-bottom:8px}" +
    ".fui-input::placeholder{direction:rtl;text-align:right}" +
    ".fui-input:focus{outline:none;border-color:#1a73e8}" +
    ".fui-btn-primary{background:#1a73e8;color:#fff;border-color:#1a73e8}" +
    ".fui-btn-primary:hover{background:#1765cc}" +
    ".fui-btn-secondary{background:#fff;color:#1a73e8;border-color:#1a73e8}" +
    ".fui-btn-secondary:hover{background:#e8f0fe}" +
    ".fui-row{display:flex;gap:8px;margin-top:8px}" +
    ".fui-row .fui-btn{flex:1}" +
    ".fui-error{background:#fce8e6;color:#c5221f;padding:8px 12px;border-radius:4px;font-size:13px;margin-top:8px;display:none}" +
    ".fui-spinner{text-align:center;margin-top:8px;display:none}" +
    ".fui-spinner::after{content:'';display:inline-block;width:20px;height:20px;border:2px solid #dadce0;border-top-color:#1a73e8;border-radius:50%;animation:fui-spin .6s linear infinite}" +
    "@keyframes fui-spin{to{transform:rotate(360deg)}}";
  container.appendChild(style);

  var wrap = document.createElement("div");
  wrap.className = "fui-container";

  // Google button
  var googleBtn = document.createElement("button");
  googleBtn.type = "button";
  googleBtn.className = "fui-btn fui-btn-google";
  googleBtn.innerHTML =
    '<img src="https://www.gstatic.com/firebasejs/ui/2.0.0/images/auth/google.svg" alt="Google">' +
    "<span>כניסה עם Google</span>";
  wrap.appendChild(googleBtn);

  // Divider
  var divider = document.createElement("div");
  divider.className = "fui-divider";
  divider.innerHTML = "<hr><span>או</span><hr>";
  wrap.appendChild(divider);

  // Email input
  var emailInput = document.createElement("input");
  emailInput.type = "email";
  emailInput.className = "fui-input";
  emailInput.placeholder = "אימייל";
  wrap.appendChild(emailInput);

  // Password input
  var passInput = document.createElement("input");
  passInput.type = "password";
  passInput.className = "fui-input";
  passInput.placeholder = "סיסמה";
  wrap.appendChild(passInput);

  // Buttons row
  var row = document.createElement("div");
  row.className = "fui-row";
  var signInBtn = document.createElement("button");
  signInBtn.type = "button";
  signInBtn.className = "fui-btn fui-btn-primary";
  signInBtn.textContent = "כניסה";
  var signUpBtn = document.createElement("button");
  signUpBtn.type = "button";
  signUpBtn.className = "fui-btn fui-btn-secondary";
  signUpBtn.textContent = "הרשמה";
  row.appendChild(signInBtn);
  row.appendChild(signUpBtn);
  wrap.appendChild(row);

  // Error
  var errorDiv = document.createElement("div");
  errorDiv.className = "fui-error";
  wrap.appendChild(errorDiv);

  // Spinner
  var spinner = document.createElement("div");
  spinner.className = "fui-spinner";
  wrap.appendChild(spinner);

  container.appendChild(wrap);

  function setLoading(on) {
    spinner.style.display = on ? "block" : "none";
    googleBtn.disabled = on;
    signInBtn.disabled = on;
    signUpBtn.disabled = on;
  }

  function showError(msg) {
    errorDiv.textContent = msg;
    errorDiv.style.display = "block";
  }

  function clearError() {
    errorDiv.style.display = "none";
  }

  function handleError(err) {
    var code = err.code || "";
    var messages = {
      "auth/invalid-email": "כתובת אימייל לא תקינה",
      "auth/user-disabled": "המשתמש חסום",
      "auth/user-not-found": "משתמש לא נמצא",
      "auth/wrong-password": "סיסמה שגויה",
      "auth/invalid-credential": "פרטי כניסה שגויים",
      "auth/email-already-in-use": "כתובת האימייל כבר בשימוש",
      "auth/weak-password": "הסיסמה חלשה מדי (מינימום 6 תווים)",
      "auth/too-many-requests": "יותר מדי ניסיונות, נסה שוב מאוחר יותר",
      "auth/popup-closed-by-user": "",
      "auth/cancelled-popup-request": "",
    };
    var msg = messages[code];
    if (msg === "") return; // silently ignore popup close
    if (msg === undefined) msg = err.message || "שגיאה בכניסה";
    showError(msg);
  }

  googleBtn.addEventListener("click", function () {
    clearError();
    setLoading(true);
    signInWithPopup(auth, googleProvider)
      .catch(handleError)
      .finally(function () { setLoading(false); });
  });

  signInBtn.addEventListener("click", function () {
    clearError();
    var email = emailInput.value.trim();
    var pass = passInput.value;
    if (!email || !pass) { showError("יש למלא אימייל וסיסמה"); return; }
    setLoading(true);
    signInWithEmailAndPassword(auth, email, pass)
      .catch(handleError)
      .finally(function () { setLoading(false); });
  });

  signUpBtn.addEventListener("click", function () {
    clearError();
    var email = emailInput.value.trim();
    var pass = passInput.value;
    if (!email || !pass) { showError("יש למלא אימייל וסיסמה"); return; }
    setLoading(true);
    createUserWithEmailAndPassword(auth, email, pass)
      .catch(handleError)
      .finally(function () { setLoading(false); });
  });
};

// ── Firestore ─────────────────────────────────────────

window.firestoreGet = function (uid) {
  var docRef = doc(db, "users", uid);
  return getDoc(docRef).then(function (snap) {
    if (snap.exists()) {
      return JSON.stringify(snap.data());
    }
    return null;
  });
};

window.firestoreSet = function (uid, jsonData) {
  var data = JSON.parse(jsonData);
  var docRef = doc(db, "users", uid);
  return setDoc(docRef, data).catch(function (err) {
    console.error("firestoreSet error:", err);
  });
};

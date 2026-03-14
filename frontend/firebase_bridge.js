// firebase_bridge.js — thin JS shim called from Rust via wasm-bindgen

const FIREBASE_CONFIG = {
  apiKey: "AIzaSyAzamvDrN5BeMsm3lCxnAuW0v-M0hc9HxI",
  authDomain: "degree-planer.firebaseapp.com",
  projectId: "degree-planer",
  storageBucket: "degree-planer.appspot.com",
  messagingSenderId: "267979518498",
  appId: "1:267979518498:web:default",
};

// Initialize Firebase (idempotent)
if (!firebase.apps.length) {
  firebase.initializeApp(FIREBASE_CONFIG);
}

// ── Auth ──────────────────────────────────────────────

// Start FirebaseUI inside the given DOM element ID.
// Returns nothing — auth result is delivered via onAuthChange.
window.startAuthUI = function (elementId) {
  const uiConfig = {
    signInFlow: "popup",
    signInOptions: [
      firebase.auth.GoogleAuthProvider.PROVIDER_ID,
      firebase.auth.EmailAuthProvider.PROVIDER_ID,
    ],
    callbacks: {
      signInSuccessWithAuthResult: function () {
        return false; // don't redirect
      },
    },
    credentialHelper: firebaseui.auth.CredentialHelper.NONE,
  };
  let ui = firebaseui.auth.AuthUI.getInstance();
  if (!ui) {
    ui = new firebaseui.auth.AuthUI(firebase.auth());
  }
  ui.start("#" + elementId, uiConfig);
};

// Register a callback for auth state changes.
// `cb` receives a JSON string: { uid, displayName, email } or null.
window.onAuthChange = function (cb) {
  firebase.auth().onAuthStateChanged(function (user) {
    if (user) {
      cb(JSON.stringify({
        uid: user.uid,
        displayName: user.displayName || "",
        email: user.email || "",
      }));
    } else {
      cb(null);
    }
  });
};

window.signOutUser = function () {
  return firebase.auth().signOut();
};

// ── Firestore ─────────────────────────────────────────

// Read user document. Returns a Promise<string|null> (JSON string or null if not found).
window.firestoreGet = function (uid) {
  return firebase.firestore().collection("users").doc(uid)
    .get()
    .then(function (doc) {
      if (doc.exists) {
        return JSON.stringify(doc.data());
      }
      return null;
    });
};

// Write user document. `jsonData` is a JSON string.
// Returns a Promise<void>.
window.firestoreSet = function (uid, jsonData) {
  var data = JSON.parse(jsonData);
  return firebase.firestore().collection("users").doc(uid)
    .set(data)
    .catch(function (err) {
      console.error("firestoreSet error:", err);
    });
};

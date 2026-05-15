// android/app/src/main/java/com/rustdialer/MainActivity.java
// ─────────────────────────────────────────────────────────────────────────────
// Thin Java Activity that:
//   1. Loads the Rust shared library (librust_dialer.so)
//   2. Sets environment variables so Rust knows Android storage paths
//   3. Requests runtime permissions (CALL_PHONE)
//   4. Delegates everything else to the Rust/Slint NativeActivity
//
// This file is intentionally minimal — all UI and logic lives in Rust.
// ─────────────────────────────────────────────────────────────────────────────
package com.rustdialer;

import android.Manifest;
import android.app.NativeActivity;
import android.content.pm.PackageManager;
import android.os.Bundle;
import android.util.Log;
import androidx.core.app.ActivityCompat;
import androidx.core.content.ContextCompat;

public class MainActivity extends NativeActivity {

    private static final String TAG              = "RustDialer";
    private static final int    REQ_CALL_PHONE   = 1001;

    static {
        // Load the Rust shared library.
        // The .so filename matches the `name` field in Cargo.toml [lib].
        System.loadLibrary("rust_dialer");
        Log.i(TAG, "librust_dialer.so loaded");
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        // Pass the internal files directory to Rust so it can store data there.
        // This is the standard Android internal storage path:
        //   /data/data/com.rustdialer.app/files/
        String dataDir = getFilesDir().getAbsolutePath();
        setEnv("ANDROID_DATA_DIR", dataDir);
        setEnv("RUST_DIALER_PKG",  getPackageName());

        Log.i(TAG, "Data dir set to: " + dataDir);

        super.onCreate(savedInstanceState);

        // Request CALL_PHONE permission at runtime (required for Android 6+)
        requestCallPermission();
    }

    /** Set an environment variable visible to the Rust process. */
    private void setEnv(String key, String value) {
        try {
            android.system.Os.setenv(key, value, true);
        } catch (Exception e) {
            Log.w(TAG, "setenv failed for " + key + ": " + e.getMessage());
        }
    }

    private void requestCallPermission() {
        if (ContextCompat.checkSelfPermission(this, Manifest.permission.CALL_PHONE)
                != PackageManager.PERMISSION_GRANTED) {
            ActivityCompat.requestPermissions(
                this,
                new String[]{ Manifest.permission.CALL_PHONE },
                REQ_CALL_PHONE
            );
        }
    }

    @Override
    public void onRequestPermissionsResult(
            int requestCode,
            String[] permissions,
            int[] grantResults) {
        super.onRequestPermissionsResult(requestCode, permissions, grantResults);
        if (requestCode == REQ_CALL_PHONE) {
            boolean granted = grantResults.length > 0
                           && grantResults[0] == PackageManager.PERMISSION_GRANTED;
            Log.i(TAG, "CALL_PHONE permission: " + (granted ? "granted" : "denied"));
            // Notify Rust side via JNI (optional — the UI handles the denied state)
            onPermissionResult(granted ? 1 : 0);
        }
    }

    /** JNI method — implemented in Rust to receive permission results. */
    private native void onPermissionResult(int granted);
}

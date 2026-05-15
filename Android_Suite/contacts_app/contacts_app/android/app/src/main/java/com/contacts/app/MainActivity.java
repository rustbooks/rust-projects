// android/app/src/main/java/com/contacts/app/MainActivity.java
// Minimal Android Activity — just loads the Rust .so via Slint's NativeActivity wrapper.
// All app logic lives in Rust. This Java file is boilerplate required by Android.

package com.contacts.app;

import android.app.NativeActivity;
import android.os.Bundle;

/**
 * Slint on Android uses NativeActivity.
 * The actual app logic is in the Rust libcontacts_app.so loaded below.
 *
 * For production: extend to handle:
 *   - Deep links (vCard open intent)
 *   - Share sheet (export via Android share)
 *   - Back gesture handling
 */
public class MainActivity extends NativeActivity {

    static {
        // Load the Rust shared library
        System.loadLibrary("contacts_app");
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        // Slint's android_activity crate handles everything from here.
    }
}
